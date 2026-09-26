
int __thiscall FUN_004fd840(void *this,int *param_1)

{
  int *piVar1;
  void *this_00;
  int iVar2;
  int **ppiVar3;
  undefined4 local_2c [7];
  int local_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  piVar1 = param_1;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00640318;
  pvStack_c = ExceptionList;
  iVar2 = 1;
  ExceptionList = &pvStack_c;
  *param_1 = 0;
  param_1 = (int *)0x0;
  FUN_00502db0(local_2c,this,3);
  local_4 = 0;
  FUN_00513120((int)local_2c);
  do {
    if (local_10 == 0) break;
    ppiVar3 = &param_1;
    this_00 = (void *)FUN_0052bed0((int)local_2c);
    iVar2 = FUN_005007a0(this_00,ppiVar3);
    if (iVar2 != 0) {
      *piVar1 = *piVar1 + (int)param_1;
    }
    FUN_005130d0((int)local_2c);
  } while (iVar2 != 0);
  local_4 = 0xffffffff;
  FUN_00502eb0(local_2c);
  ExceptionList = pvStack_c;
  return iVar2;
}

