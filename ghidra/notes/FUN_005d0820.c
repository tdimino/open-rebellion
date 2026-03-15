
void * __thiscall FUN_005d0820(void *this,undefined4 param_1)

{
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00654508;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_005c6150((undefined4 *)((int)this + 0x18));
  *(undefined4 *)((int)this + 0x18) = &PTR_FUN_0066bdb8;
  *(undefined4 *)((int)this + 8) = param_1;
  *(undefined4 *)((int)this + 0x14) = 0;
  *(undefined4 *)((int)this + 4) = 0;
  *(undefined ***)this = &PTR_FUN_0066d0a8;
  DAT_006bc978 = 1;
  local_4 = 0;
  *(undefined4 *)((int)this + 0xc) = 0;
  *(undefined4 *)((int)this + 0x24) = 0;
  *(undefined4 *)((int)this + 0x28) = 0;
  *(undefined4 *)((int)this + 0x10) = 0;
  *(undefined4 *)((int)this + 0x34) = 0;
  FUN_005cfe80(this,2);
  *(undefined4 *)((int)this + 0x30) = 0;
  ExceptionList = local_c;
  return this;
}

