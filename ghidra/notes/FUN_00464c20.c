
UINT FUN_00464c20(int *param_1,uint param_2,HPALETTE param_3,HWND param_4)

{
  void *this;
  int *piVar1;
  short sVar2;
  BOOL BVar3;
  UINT UVar4;
  undefined4 *puVar5;
  void *this_00;
  int *piVar6;
  int iVar7;
  UINT local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  piVar1 = param_1;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006316d8;
  local_c = ExceptionList;
  this_00 = (void *)0x0;
  local_10 = 0;
  if (param_2 < 0x11) {
    if (param_2 == 0x10) {
      iVar7 = 5;
      piVar6 = param_1 + 0x57;
      ExceptionList = &local_c;
      do {
        BVar3 = DeleteObject((HGDIOBJ)*piVar6);
        if (BVar3 != 0) {
          *piVar6 = 0;
        }
        piVar6 = piVar6 + 1;
        iVar7 = iVar7 + -1;
      } while (iVar7 != 0);
      UVar4 = FUN_00606650(piVar1,0x10,param_3,param_4);
      ExceptionList = local_c;
      return UVar4;
    }
    if (param_2 == 7) {
      ExceptionList = &local_c;
      SetFocus(*(HWND *)(param_1[0x53] + 0x18));
      ExceptionList = local_c;
      return 0;
    }
LAB_00464cbf:
    ExceptionList = &local_c;
    local_10 = FUN_00606650(param_1,param_2,param_3,param_4);
  }
  else {
    if (param_2 == 0x407) {
      ExceptionList = &local_c;
      FUN_004ece30(&param_4);
      piVar1 = param_1;
      local_4 = 2;
      if ((((short)param_3 == 0xcb) && ((HWND)param_1[0x5e] != param_4)) &&
         (iVar7 = FUN_00429440((void *)param_1[0x55],(uint *)(param_1 + 0x5e)), iVar7 != 0)) {
        (**(code **)(*piVar1 + 0x30))();
      }
    }
    else {
      if (param_2 != 0x408) goto LAB_00464cbf;
      if ((short)param_3 != 0xcb) {
        return 0;
      }
      iVar7 = param_1[0x53];
      this = *(void **)(param_1[0x54] + 0xa0);
      ExceptionList = &local_c;
      sVar2 = FUN_005f3040(iVar7 + 0x98);
      if (sVar2 != 0) {
        iVar7 = FUN_00609650((void *)piVar1[0x54],iVar7 + 0x98,0,0);
        this_00 = (void *)FUN_0060a860(this,iVar7);
      }
      if (this_00 == (void *)0x0) {
        FUN_004ece30(&param_1);
        local_4 = 1;
        FUN_004f26d0(piVar1 + 0x5e,&param_1);
      }
      else {
        puVar5 = (undefined4 *)FUN_0042d170(this_00,&param_2);
        local_4 = 0;
        FUN_004f26d0(piVar1 + 0x5e,puVar5);
      }
    }
    local_4 = 0xffffffff;
    FUN_00619730();
  }
  ExceptionList = local_c;
  return local_10;
}

