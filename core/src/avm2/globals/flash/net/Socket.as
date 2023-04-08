package flash.net {
    import flash.events.EventDispatcher;
    import flash.utils.ByteArray;

    import __ruffle__.stub_constructor;
    import __ruffle__.stub_method;
    import __ruffle__.stub_getter;
    import __ruffle__.stub_setter;

    public class Socket extends EventDispatcher {
        public function Socket(host: String = null, port: int = 0) {
            stub_constructor("flash.net.Socket");
        }

        public function close(): void {
            stub_method("flash.net.Socket", "close");
        }

        public function connect(host: String, port: int): void {
            stub_method("flash.net.Socket", "connect");
        }

        public function flush(): void {
            stub_method("flash.net.Socket", "flush");
        }

        public function readBoolean(): Boolean {
            stub_method("flash.net.Socket", "readBoolean");
            return false;
        }

        public function readByte(): int {
            stub_method("flash.net.Socket", "readByte");
            return 0;
        }

        public function readBytes(bytes: ByteArray, offset: uint = 0, length: uint = 0): void {
            stub_method("flash.net.Socket", "readBytes");
        }

        public function readDouble(): Number {
            stub_method("flash.net.Socket", "readDouble");
            return 0;
        }

        public function readFloat(): Number {
            stub_method("flash.net.Socket", "readFloat");
            return 0;
        }

        public function readInt(): int {
            stub_method("flash.net.Socket", "readInt");
            return 0;
        }

        public function readMultiByte(length: uint, charSet: String): String {
            stub_method("flash.net.Socket", "readMultiByte");
            return "";
        }

        public function readObject(): * {
            stub_method("flash.net.Socket", "readObject");
            return null;
        }

        public function readShort(): int {
            stub_method("flash.net.Socket", "readShort");
            return 0;
        }

        public function readUnsignedByte(): uint {
            stub_method("flash.net.Socket", "readUnsignedByte");
            return 0;
        }

        public function readUnsignedInt(): uint {
            stub_method("flash.net.Socket", "readUnsignedInt");
            return 0;
        }

        public function readUnsignedShort(): uint {
            stub_method("flash.net.Socket", "readUnsignedShort");
            return 0;
        }

        public function readUTF(): String {
            stub_method("flash.net.Socket", "readUTF");
            return "";
        }

        public function readUTFBytes(length: uint): String {
            stub_method("flash.net.Socket", "readUTFBytes");
            return "";
        }

        public function writeBoolean(value: Boolean): void {
            stub_method("flash.net.Socket", "writeBoolean");
        }

        public function writeByte(value: int): void {
            stub_method("flash.net.Socket", "writeByte");
        }

        public function writeBytes(bytes: ByteArray, offset: uint = 0, length: uint = 0): void {
            stub_method("flash.net.Socket", "writeBytes");
        }

        public function writeDouble(value: Number): void {
            stub_method("flash.net.Socket", "writeDouble");
        }

        public function writeFloat(value: Number): void {
            stub_method("flash.net.Socket", "writeFloat");
        }

        public function writeInt(value: int): void {
            stub_method("flash.net.Socket", "writeInt");
        }

        public function writeMultiByte(value: String, charSet: String): void {
            stub_method("flash.net.Socket", "writeMultiByte");
        }

        public function writeObject(object: *): void {
            stub_method("flash.net.Socket", "writeObject");
        }

        public function writeShort(value: int): void {
            stub_method("flash.net.Socket", "writeShort");
        }

        public function writeUnsignedInt(value: uint): void {
            stub_method("flash.net.Socket", "writeUnsignedInt");
        }

        public function writeUTF(value: String): void {
            stub_method("flash.net.Socket", "writeUTF");
        }

        public function writeUTFBytes(value: String): void {
            stub_method("flash.net.Socket", "writeUTFBytes");
        }

        public function send(object: *): void {
            stub_method("flash.net.Socket", "send");
        }

        public function get bytesAvailable():uint {
            stub_getter("flash.net.Socket", "bytesAvailable");
            return 0;
        }

        public function get bytesPending():uint {
            stub_getter("flash.net.Socket", "bytesPending");
            return 0;
        }

        public function get connected():Boolean {
            stub_getter("flash.net.Socket", "connected");
            return false;
        }

        public function get endian():String {
            stub_getter("flash.net.Socket", "endian");
            return "";
        }

        public function set endian(type:String):void {
            stub_setter("flash.net.Socket", "endian");
        }

        public function get localAddress():String {
            stub_getter("flash.net.Socket", "localAddress");
            return "";
        }

        public function get localPort():int {
            stub_getter("flash.net.Socket", "localPort");
            return 0;
        }

        public function get objectEncoding():uint {
            stub_getter("flash.net.Socket", "objectEncoding");
            return 0;
        }

        public function set objectEncoding(version:uint):void {
            stub_setter("flash.net.Socket", "objectEncoding");
        }

        public function get remoteAddress():String {
            stub_getter("flash.net.Socket", "remoteAddress");
            return "";
        }

        public function get remotePort():int {
            stub_getter("flash.net.Socket", "remotePort");
            return 0;
        }

        public function get timeout():uint {
            stub_getter("flash.net.Socket", "timeout");
            return 0;
        }

        public function set timeout(value:uint):void {
            stub_setter("flash.net.Socket", "timeout");
        }
    }
}
