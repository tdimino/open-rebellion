
bool FUN_005560b0(int *param_1,int *param_2,int *param_3)

{
  int *piVar1;
  int *piVar2;
  int *piVar3;
  bool bVar4;
  int iVar5;
  uint *puVar6;
  undefined3 extraout_var;
  bool bVar7;
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_006495f8;
  local_c = ExceptionList;
  bVar7 = true;
  local_14 = 1;
  local_10 = 0xffffffff;
  local_4 = 0;
  ExceptionList = &local_c;
  FUN_00520580(param_2,&local_14);
  local_4 = 0xffffffff;
  FUN_00619730();
  piVar3 = param_3;
  piVar2 = param_1;
  *param_3 = 0;
  if ((*(byte *)(param_1 + 0x14) & 0x10) != 0) {
    piVar1 = param_1 + 0x11;
    param_1 = (int *)0x0;
    *param_3 = *piVar1;
    FUN_004ece30(&param_3);
    local_4 = 1;
    FUN_004ece30(&param_2);
    local_4 = CONCAT31(local_4._1_3_,2);
    iVar5 = (**(code **)(*piVar2 + 0xc))(&param_3);
    bVar7 = iVar5 != 0;
    FUN_004f26d0(&param_2,piVar2 + 0xf);
    puVar6 = FUN_004ece40((uint *)&param_3);
    if (puVar6 != (uint *)0x0) {
      puVar6 = FUN_004ece40((uint *)&param_2);
      if ((puVar6 != (uint *)0x0) && (param_3 != param_2)) {
        bVar4 = FUN_00555d30(piVar2,(uint *)&param_2,(int *)&param_3,&param_1);
        if ((CONCAT31(extraout_var,bVar4) == 0) || (!bVar7)) {
          bVar7 = false;
        }
        else {
          bVar7 = true;
        }
        *piVar3 = *piVar3 + (int)param_1;
      }
    }
    local_4 = CONCAT31(local_4._1_3_,1);
    FUN_00619730();
    local_4 = 0xffffffff;
    FUN_00619730();
  }
  ExceptionList = local_c;
  return bVar7;
}

