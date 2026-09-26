
int __thiscall FUN_004ef340(void *this,int param_1)

{
  int iVar1;
  int iVar2;
  int unaff_EBX;
  undefined4 unaff_retaddr;
  void *local_14;
  undefined4 local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_0063eef8;
  pvStack_c = ExceptionList;
  local_14 = (void *)0x1;
  local_10 = 0xffffffff;
  local_4 = 0;
  ExceptionList = &pvStack_c;
  iVar1 = (**(code **)(*(int *)this + 0x2d4))(param_1,&local_14);
  if ((iVar1 != 0) && (unaff_EBX == -1)) {
    local_4 = 0;
    iVar2 = FUN_0053ebb0(*(undefined4 *)(param_1 + 0xc),&local_4);
    iVar1 = 0;
    if (iVar2 != 0) {
      iVar1 = (**(code **)(*(int *)this + 0x2f8))(local_4,unaff_retaddr,param_1);
    }
  }
  pvStack_c = (void *)0xffffffff;
  FUN_00619730();
  ExceptionList = local_14;
  return iVar1;
}

