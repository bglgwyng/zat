using System;
using System.Collections.Generic;

namespace Example
{
    public class Service
    {
        public string Name { get; set; }
        private string _secret;
        protected int Value { get; set; }

        public Service(string name)
        {
            Name = name;
            _secret = "";
        }

        public string GetName()
        {
            return Name;
        }

        private void Internal()
        {
            // hidden
        }
    }

    public interface IReader
    {
        int Read(byte[] buffer);
        void Close();
    }

    public enum Color
    {
        Red,
        Green,
        Blue
    }

    public struct Point
    {
        public int X;
        public int Y;
    }

    public record Person(string Name, int Age);

    public abstract class Base
    {
        public abstract void Run();
    }
}
