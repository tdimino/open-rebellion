
int __thiscall FUN_004ff690(void *this,int param_1,void *param_2)

{
  bool bVar1;
  undefined3 extraout_var;
  int iVar2;
  int *piVar3;
  int iVar4;
  undefined4 local_2c [7];
  int local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00640668;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  bVar1 = FUN_004fba70(this,param_1,param_2);
  iVar4 = CONCAT31(extraout_var,bVar1);
  FUN_00502db0(local_2c,this,3);
  local_4 = 0;
  FUN_00513120((int)local_2c);
  while (local_10 != 0) {
    iVar2 = FUN_0052bed0((int)local_2c);
    if (((*(uint *)(iVar2 + 0x50) >> 2 & 1) != 0) &&
       (iVar2 = FUN_0052bed0((int)local_2c), (*(uint *)(iVar2 + 0x50) >> 5 & 1) == 0)) {
      piVar3 = (int *)FUN_0052bed0((int)local_2c);
      iVar2 = (**(code **)(*piVar3 + 0x34))(0);
      if (iVar2 == 0) {
        piVar3 = (int *)FUN_0052bed0((int)local_2c);
        iVar2 = (**(code **)(*piVar3 + 0xac))(5,param_2);
        if ((iVar2 == 0) || (iVar4 == 0)) {
          iVar4 = 0;
        }
        else {
          iVar4 = 1;
        }
      }
    }
    FUN_005130d0((int)local_2c);
  }
  local_4 = 0xffffffff;
  FUN_00502eb0(local_2c);
  ExceptionList = pvStack_c;
  return iVar4;
}

