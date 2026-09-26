
undefined4 __thiscall FUN_005da080(void *this,int param_1)

{
  int iVar1;
  void *local_14;
  code *pcStack_10;
  undefined *puStack_c;
  undefined4 local_8;
  
  local_8 = 0xffffffff;
  puStack_c = &DAT_0066d4a8;
  pcStack_10 = FUN_0061a414;
  local_14 = ExceptionList;
  if (param_1 == 0) {
    param_1 = *(int *)((int)this + 0xc);
  }
  ExceptionList = &local_14;
  iVar1 = (**(code **)(**(int **)((int)this + 4) + 0x30))(*(int **)((int)this + 4));
  *(int *)((int)this + 0x2c) = iVar1;
  if (iVar1 != 0) {
    FUN_005d93f0(*(int *)((int)this + 0x10),iVar1);
  }
  local_8 = 0;
  iVar1 = (**(code **)(**(int **)((int)this + 4) + 0x34))(*(int **)((int)this + 4),param_1);
  *(int *)((int)this + 0x2c) = iVar1;
  if (iVar1 != 0) {
    FUN_005d93f0(*(int *)((int)this + 0x10),iVar1);
  }
  ExceptionList = local_14;
  return 1;
}

