
int __thiscall FUN_00509530(void *this,undefined4 param_1,int *param_2)

{
  int *piVar1;
  int *piVar2;
  int iVar3;
  void *this_00;
  int iVar4;
  int iVar5;
  int **ppiVar6;
  undefined4 local_2c [7];
  int local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  piVar2 = param_2;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00641998;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  iVar3 = FUN_00509330(this,1,param_2);
  FUN_004ffef0(local_2c,this,1,param_1);
  local_4 = 0;
  FUN_00513120((int)local_2c);
  do {
    if ((iVar3 == 0) || (local_10 == 0)) {
      local_4 = 0xffffffff;
      FUN_004fff70(local_2c);
      ExceptionList = pvStack_c;
      return iVar3;
    }
    ppiVar6 = &param_2;
    param_2 = (int *)0x0;
    iVar3 = 1;
    this_00 = (void *)FUN_0052bed0((int)local_2c);
    iVar3 = FUN_004fd790(this_00,iVar3,(int *)ppiVar6);
    if ((iVar3 != 0) && (param_2 != (int *)0x0)) {
      piVar1 = (int *)*piVar2;
      if (piVar1 != (int *)0x0) {
        iVar4 = (**(code **)(*piVar1 + 0x1f0))();
        iVar5 = (**(code **)(*param_2 + 0x1f0))();
        if (iVar5 <= iVar4) goto LAB_005095d5;
      }
      *piVar2 = (int)param_2;
    }
LAB_005095d5:
    FUN_005130d0((int)local_2c);
  } while( true );
}

