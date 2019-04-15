using System;
using System.Xml.Serialization;

namespace xml_builder
{
    [Serializable]
    public class Asset
    {
        [XmlAttribute]
        public string AssetPath;
    }
}