
bool __thiscall FUN_00528b30(void *this,int param_1,int param_2,void *param_3,int *param_4)

{
  int iVar1;
  int *piVar2;
  int iVar3;
  int iVar4;
  bool bVar5;
  undefined4 local_34;
  undefined4 local_30;
  undefined4 local_2c [8];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_006446c0;
  local_c = ExceptionList;
  bVar5 = true;
  local_34 = 1;
  local_30 = 0xffffffff;
  local_4 = 0;
  ExceptionList = &local_c;
  FUN_00520580(param_3,&local_34);
  local_4 = 0xffffffff;
  FUN_00619730();
  *param_4 = 0;
  iVar1 = FUN_00528100(this,0);
  if (iVar1 == 0) {
    iVar1 = FUN_00528100(this,1);
    if (iVar1 == 0) {
      local_34 = 0xa0;
      local_30 = 3;
      local_4 = 1;
      FUN_00520580(param_3,&local_34);
      local_4 = 0xffffffff;
      FUN_00619730();
    }
    else {
      local_34 = 0xa0;
      local_30 = 4;
      local_4 = 2;
      FUN_00520580(param_3,&local_34);
      local_4 = 0xffffffff;
      FUN_00619730();
    }
  }
  else {
    iVar1 = FUN_0052bed0((int)this);
    bVar5 = iVar1 != 0;
    if (iVar1 != 0) {
      FUN_0052cda0(local_2c,iVar1);
      local_4 = 3;
      FUN_00513120((int)local_2c);
      iVar1 = FUN_0052bed0((int)local_2c);
      while (iVar1 != 0) {
        piVar2 = (int *)FUN_0052bed0((int)local_2c);
        iVar1 = FUN_0052bde0(this,piVar2);
        if ((iVar1 != 0) &&
           (iVar1 = FUN_0052bed0((int)local_2c), (*(uint *)(iVar1 + 0x60) & 1) == 0)) {
          iVar1 = param_1;
          if ((param_2 != 0) && (iVar3 = FUN_0052bed0((int)local_2c), *(int *)(iVar3 + 0x5c) != 0))
          {
            iVar1 = FUN_0052bed0((int)local_2c);
            iVar1 = *(int *)(iVar1 + 0x5c);
            iVar3 = FUN_0052bed0((int)local_2c);
            iVar3 = FUN_00520b70(iVar3);
            iVar4 = FUN_004fd340();
            iVar1 = iVar3 + iVar4 + (param_1 - iVar1);
          }
          iVar3 = FUN_0052bed0((int)local_2c);
          iVar3 = FUN_00520b70(iVar3);
          *param_4 = *param_4 + iVar1 / iVar3;
        }
        FUN_005130d0((int)local_2c);
        iVar1 = FUN_0052bed0((int)local_2c);
      }
      local_4 = 0xffffffff;
      FUN_0052cec0(local_2c);
    }
  }
  ExceptionList = local_c;
  return bVar5;
}

