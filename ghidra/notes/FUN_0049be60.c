
void * __thiscall FUN_0049be60(void *this,undefined4 *param_1,int param_2)

{
  undefined4 uVar1;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00636eef;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_004c4d90(this,param_2);
  local_4 = 0;
  FUN_004ece30((undefined4 *)((int)this + 0x44));
  local_4._0_1_ = 1;
  FUN_005f2f50((undefined4 *)((int)this + 0x54));
  local_4._0_1_ = 2;
  FUN_005f2f50((undefined4 *)((int)this + 0x60));
  local_4._0_1_ = 3;
  FUN_004f5510((undefined4 *)((int)this + 0x6c));
  local_4._0_1_ = 4;
  FUN_004f5510((undefined4 *)((int)this + 0x74));
  local_4 = CONCAT31(local_4._1_3_,5);
  *(undefined ***)this = &PTR_FUN_0065bc40;
  uVar1 = FUN_0048bc60();
  *(undefined4 *)((int)this + 0x34) = uVar1;
  FUN_004f26d0((undefined4 *)((int)this + 0x44),param_1);
  *(undefined4 *)((int)this + 0x50) = 0;
  if (param_2 == 1) {
    *(undefined4 *)((int)this + 0x48) = 1;
  }
  else if (param_2 == 2) {
    *(undefined4 *)((int)this + 0x48) = 2;
  }
  else {
    *(undefined4 *)((int)this + 0x48) = 0;
  }
  uVar1 = FUN_0049c0d0(this);
  *(undefined4 *)((int)this + 0x1c) = uVar1;
  ExceptionList = local_c;
  return this;
}

