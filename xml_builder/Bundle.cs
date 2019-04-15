using System;
using System.Xml.Serialization;

namespace xml_builder
{
    [Serializable]
    public class Bundle
    {
        [XmlAttribute]
        public string Filename;
        
        [XmlAttribute]
        public int DownloadSize;
        
        public Asset[] Assets;
    }
}