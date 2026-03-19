import { FailOpenFileError } from '@common/errors';
import { logger } from '@common/Logger';
import { GroupedTag } from '@common/logging/GroupedTag';
import { PieLoadingPartType } from '@common/types/LoadingTypes';
import type { ModelJsonRoot } from '@common/types/ModelTypes';
import { currentModelBaseVersion, currentModelVersion } from '@common/types/ModelTypes';

import type { FontMigration } from './FontMigration';
import type { Migration, MigrationContext } from './MigrationDefs';

const TAG = new GroupedTag('MODEL', 'PieFileMigrator');

export class PieFileMigrator {
  private static _migrations: Migration[] = [];
  private static _unversionedMigration: Migration | null = null;
  private static _fontMigration: FontMigration | null = null;

  public static addMigration(migration: Migration) {
    this._migrations.push(migration);
  }

  public static setUnversionedMigration(migration: Migration) {
    this._unversionedMigration = migration;
  }

  public static setFontMigration(migration: FontMigration) {
    this._fontMigration = migration;
  }

  public async migrate(context: MigrationContext, data: ModelJsonRoot, metadata: any[]): Promise<[ModelJsonRoot, any]> {
    try {
      const version = data.modelVersion || 1;
      const migrations = PieFileMigrator._migrations.sort((a, b) => a.version - b.version);
      const lastMigrationVersion = migrations.slice(-1)[0].version;

      if (currentModelVersion > lastMigrationVersion) {
        logger.error(
          TAG,
          `The MigrationV${
            currentModelVersion < 100 ? 0 : ''
          }${currentModelVersion} file must be imported in ImportMigrations.ts file.`,
          {
            currentModelVersion,
            lastMigrationVersion,
          },
        );
      }

      for (let i = 0; i < migrations.length; i++) {
        const migration = migrations[i];

        if (version < migration.version) {
          logger.info(TAG, `migrating data from ${version} to ${migration.version}`);
          [data, metadata] = await migration.up(context, data, metadata);
          await context.server.loading.setLoadingPieProgress(PieLoadingPartType.MIGRATE, (i + 1) / migrations.length);
        }
      }

      if (PieFileMigrator._unversionedMigration) {
        logger.info(TAG, `migrating data by unversionedMigrator`);
        [data, metadata] = await PieFileMigrator._unversionedMigration.up(context, data, metadata);
      }
      if (PieFileMigrator._fontMigration) {
        logger.info(TAG, `migrating data by fontMigrator`);
        [data, metadata] = await PieFileMigrator._fontMigration.migrate(context, data, metadata);
      }

      data.modelVersion = currentModelVersion;
      data.baseModelVersion = currentModelBaseVersion;
    } catch (ex) {
      throw new FailOpenFileError(ex);
    }

    return [data, metadata];
  }
}
