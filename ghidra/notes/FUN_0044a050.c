
void FUN_0044a050(HDC param_1,int *param_2,int *param_3)

{
  int *piVar1;
  int *piVar2;
  char *pcVar3;
  undefined4 local_74 [11];
  undefined4 local_48;
  undefined4 local_40 [11];
  undefined4 local_14;
  void *pvStack_c;
  undefined1 *puStack_8;
  uint local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062f290;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  FUN_00601880(local_40,(char *)0x0,0,0,0,0,0,0);
  local_4 = 0;
  FUN_00601880(local_74,(char *)0x0,0,0,0,0,0,0);
  local_4 = CONCAT31(local_4._1_3_,1);
  FUN_00601c60(local_40,10);
  FUN_00601c90(local_40,0xffffff,(HWND)0x0);
  FUN_00403e90(local_40,0x10);
  local_14 = 1;
  local_40[0] = 0x67;
  FUN_00601c60(local_74,10);
  FUN_00601c90(local_74,0xffffff,(HWND)0x0);
  FUN_00403e90(local_74,0x10);
  local_48 = 1;
  local_74[0] = 0x67;
  piVar1 = (int *)(**(code **)(*param_2 + 8))();
  for (piVar2 = (int *)(**(code **)(*param_3 + 8))();
      (piVar1 != (int *)0x0 && (piVar2 != (int *)0x0));
      piVar2 = (int *)(**(code **)(*piVar2 + 0xc))()) {
    FUN_00601b30(local_40,piVar1[10],piVar1[0xb]);
    pcVar3 = (char *)FUN_00583c40((int)(piVar1 + 5));
    FUN_00601aa0(local_40,pcVar3);
    FUN_00601b80(local_40,param_1);
    FUN_00601ce0(local_40,param_1);
    FUN_00601b30(local_74,piVar2[10],piVar2[0xb]);
    pcVar3 = (char *)FUN_00583c40((int)(piVar2 + 5));
    FUN_00601aa0(local_74,pcVar3);
    FUN_00601b80(local_74,param_1);
    FUN_00601ce0(local_74,param_1);
    piVar1 = (int *)(**(code **)(*piVar1 + 0xc))();
  }
  local_4 = local_4 & 0xffffff00;
  FUN_00601990((int)local_74);
  local_4 = 0xffffffff;
  FUN_00601990((int)local_40);
  ExceptionList = pvStack_c;
  return;
}

