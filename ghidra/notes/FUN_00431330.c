
void * __thiscall FUN_00431330(void *this,undefined4 param_1)

{
  int iVar1;
  undefined4 *puVar2;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062ce2e;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_004ecc00(this);
  local_4 = 0;
  FUN_005f4950((void *)((int)this + 0x24),0);
  local_4._0_1_ = 1;
  FUN_005f4950((void *)((int)this + 0x28),0);
  local_4 = CONCAT31(local_4._1_3_,2);
  FUN_005f4950((void *)((int)this + 0x2c),0);
  *(undefined4 *)((int)this + 0x1c) = param_1;
  *(undefined ***)this = &PTR_FUN_00658d30;
  *(undefined4 *)((int)this + 0x44) = 0;
  puVar2 = (undefined4 *)((int)this + 0x48);
  for (iVar1 = 10; iVar1 != 0; iVar1 = iVar1 + -1) {
    *puVar2 = 0;
    puVar2 = puVar2 + 1;
  }
  *(undefined4 *)((int)this + 0x38) = 0;
  *(undefined4 *)((int)this + 0x3c) = 0;
  *(undefined4 *)((int)this + 0x30) = 0;
  *(undefined4 *)((int)this + 0x34) = 0;
  *(undefined4 *)((int)this + 0x20) = 0;
  ExceptionList = local_c;
  return this;
}

