
int __thiscall FUN_0058c3c0(void *this,int param_1)

{
  int iVar1;
  int iVar2;
  bool bVar3;
  int *piVar4;
  int local_40 [3];
  int local_34;
  int *local_30;
  undefined4 local_2c [7];
  int local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  uint local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0064fef0;
  pvStack_c = ExceptionList;
  iVar2 = 1;
  bVar3 = false;
  ExceptionList = &pvStack_c;
  FUN_00526fd0(local_2c,*(undefined4 *)((int)this + 0x10),1);
  local_4 = 0;
  FUN_00513120((int)local_2c);
  iVar1 = param_1;
  do {
    if ((local_10 == 0) || (bVar3)) break;
    piVar4 = &param_1;
    param_1 = 0;
    iVar2 = FUN_0052bed0((int)local_2c);
    iVar2 = FUN_00520b70(iVar2);
    iVar2 = FUN_0055ead0(iVar2,piVar4);
    if ((iVar2 != 0) && (param_1 != 0)) {
      FUN_0058bd00(local_40,(void *)((int)this + 8));
      local_4 = CONCAT31(local_4._1_3_,1);
      local_34 = FUN_0053e290(*(int *)((int)this + 0x1c) + -1);
      iVar2 = FUN_0058ba10((void *)((int)this + 8),local_40,iVar1);
      if (((iVar2 != 0) && (iVar2 != 0)) && (local_30 != (int *)0x0)) {
        iVar2 = *(int *)((int)this + 0x24) + -1;
        *(int *)((int)this + 0x24) = iVar2;
        bVar3 = iVar2 == 0;
        iVar2 = (**(code **)(*local_30 + 0xac))(0xe,iVar1);
      }
      local_4 = local_4 & 0xffffff00;
      FUN_0058bd50(local_40);
    }
    FUN_005130d0((int)local_2c);
  } while (iVar2 != 0);
  local_4 = 0xffffffff;
  FUN_005270d0(local_2c);
  ExceptionList = pvStack_c;
  return iVar2;
}

