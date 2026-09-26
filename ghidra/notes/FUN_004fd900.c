
int __thiscall FUN_004fd900(void *this,undefined4 param_1)

{
  bool bVar1;
  int iVar2;
  int *piVar3;
  int iVar4;
  undefined4 local_2c [7];
  int local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00640338;
  pvStack_c = ExceptionList;
  iVar4 = 0;
  bVar1 = true;
  ExceptionList = &pvStack_c;
  FUN_00502db0(local_2c,this,3);
  local_4 = 0;
  FUN_00513120((int)local_2c);
  do {
    if (local_10 == 0) {
      local_4 = 0xffffffff;
      FUN_00502eb0(local_2c);
      ExceptionList = pvStack_c;
      return iVar4;
    }
    iVar2 = FUN_0052bed0((int)local_2c);
    if (((*(uint *)(iVar2 + 0x50) >> 2 & 1) != 0) &&
       (iVar2 = FUN_0052bed0((int)local_2c), (*(uint *)(iVar2 + 0x50) >> 5 & 1) == 0)) {
      if (bVar1) {
        bVar1 = false;
        piVar3 = (int *)FUN_0052bed0((int)local_2c);
        iVar4 = (**(code **)(*piVar3 + 0x34))(param_1);
      }
      if (iVar4 != 0) {
        piVar3 = (int *)FUN_0052bed0((int)local_2c);
        iVar2 = (**(code **)(*piVar3 + 0x34))(param_1);
        if (iVar2 != 0) {
          piVar3 = (int *)FUN_0052bed0((int)local_2c);
          iVar2 = (**(code **)(*piVar3 + 0x34))(param_1);
          if (iVar2 <= iVar4) goto LAB_004fd9d5;
        }
        piVar3 = (int *)FUN_0052bed0((int)local_2c);
        iVar4 = (**(code **)(*piVar3 + 0x34))(param_1);
      }
    }
LAB_004fd9d5:
    FUN_005130d0((int)local_2c);
  } while( true );
}

