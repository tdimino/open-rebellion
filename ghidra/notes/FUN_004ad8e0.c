
void * __thiscall FUN_004ad8e0(void *this,undefined4 param_1,undefined4 param_2,undefined4 param_3)

{
  int iVar1;
  undefined4 *puVar2;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063881c;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_005f4950((void *)((int)this + 0x14),0);
  local_4 = 0;
  FUN_004355d0((undefined4 *)((int)this + 0x18));
  local_4._0_1_ = 1;
  FUN_004ece30((undefined4 *)((int)this + 0x28));
  local_4._0_1_ = 2;
  FUN_004ece30((undefined4 *)((int)this + 0x2c));
  local_4 = CONCAT31(local_4._1_3_,3);
  FUN_004ece30((undefined4 *)((int)this + 0x30));
  *(undefined4 *)((int)this + 4) = param_1;
  *(undefined4 *)((int)this + 8) = 0;
  *(undefined4 *)((int)this + 0xc) = param_2;
  *(undefined ***)this = &PTR_FUN_0065c0b0;
  *(undefined4 *)((int)this + 0x10) = param_3;
  puVar2 = (undefined4 *)((int)this + 0x34);
  for (iVar1 = 10; iVar1 != 0; iVar1 = iVar1 + -1) {
    *puVar2 = 0;
    puVar2 = puVar2 + 1;
  }
  ExceptionList = local_c;
  return this;
}

