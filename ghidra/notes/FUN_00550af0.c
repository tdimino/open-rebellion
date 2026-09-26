
int FUN_00550af0(int param_1,int param_2,int param_3,void *param_4)

{
  bool bVar1;
  int iVar2;
  undefined3 extraout_var;
  int iVar3;
  int iVar4;
  int iVar5;
  undefined4 local_44 [7];
  undefined4 local_28 [7];
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00648ab0;
  local_c = ExceptionList;
  iVar4 = 1;
  ExceptionList = &local_c;
  iVar2 = FUN_0054fa40(local_28);
  iVar3 = 0;
  local_4 = 0;
  FUN_0054fae0(param_4,iVar2);
  local_4 = 0xffffffff;
  FUN_0054fa90(local_28);
  FUN_0054fa40(local_44);
  local_4 = 1;
  iVar2 = param_1 + 4;
  iVar5 = param_2 + 4;
  do {
    if (8 < iVar3) break;
    iVar4 = FUN_0054fca0(local_44,iVar2,iVar5,param_3);
    if (iVar4 != 0) {
      bVar1 = FUN_0054fbb0(param_4,(int)local_44);
      if (CONCAT31(extraout_var,bVar1) != 0) {
        FUN_0054fae0(param_4,(int)local_44);
      }
    }
    iVar3 = iVar3 + 1;
    iVar5 = iVar5 + 0xc;
    iVar2 = iVar2 + 0x1c;
  } while (iVar4 != 0);
  local_4 = 0xffffffff;
  FUN_0054fa90(local_44);
  ExceptionList = local_c;
  return iVar4;
}

