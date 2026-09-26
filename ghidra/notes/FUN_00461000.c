
UINT FUN_00461000(int *param_1,uint param_2,HPALETTE param_3,HWND param_4)

{
  int *piVar1;
  short sVar2;
  int iVar3;
  undefined4 *puVar4;
  int *piVar5;
  uint *puVar6;
  void *this;
  UINT UVar7;
  uint local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  piVar1 = param_1;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00631120;
  local_c = ExceptionList;
  UVar7 = 0;
  if (param_2 == 7) {
    ExceptionList = &local_c;
    SetFocus(*(HWND *)(param_1[0x52] + 0x18));
  }
  else if (param_2 == 0x407) {
    ExceptionList = &local_c;
    FUN_004ece30(&param_2);
    piVar1 = param_1;
    local_4 = 2;
    if ((short)param_3 == 0xcb) {
      if (param_1[0x58] != param_2) {
        piVar5 = FUN_004f3220(*(int *)(param_1[0x54] + 0x9c),(uint *)(param_1 + 0x58));
        if (piVar5 != (int *)0x0) {
          puVar6 = FUN_004025b0((void *)piVar5[7],&local_10);
          local_4._0_1_ = 3;
          FUN_00429ce0((void *)piVar1[0x54],puVar6);
          local_4 = CONCAT31(local_4._1_3_,2);
          FUN_00619730();
        }
        (**(code **)(*piVar1 + 0x30))();
      }
    }
    local_4 = 0xffffffff;
    FUN_00619730();
  }
  else if (param_2 == 0x408) {
    if ((short)param_3 == 0xcb) {
      this = (void *)0x0;
      iVar3 = param_1[0x52];
      ExceptionList = &local_c;
      sVar2 = FUN_005f3040(iVar3 + 0x98);
      if (sVar2 != 0) {
        iVar3 = FUN_00609650((void *)piVar1[0x53],iVar3 + 0x98,0,0);
        this = (void *)FUN_0060a860((void *)piVar1[0x57],iVar3);
      }
      if (this == (void *)0x0) {
        FUN_004ece30(&param_1);
        local_4 = 1;
        FUN_004f26d0(piVar1 + 0x58,&param_1);
        local_4 = 0xffffffff;
        FUN_00619730();
      }
      else {
        puVar4 = FUN_0042d170(this,&param_4);
        local_4 = 0;
        FUN_004f26d0(piVar1 + 0x58,puVar4);
        local_4 = 0xffffffff;
        FUN_00619730();
      }
    }
  }
  else {
    ExceptionList = &local_c;
    UVar7 = FUN_00606650(param_1,param_2,param_3,param_4);
  }
  ExceptionList = local_c;
  return UVar7;
}

