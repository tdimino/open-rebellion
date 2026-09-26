
UINT FUN_0046e5f0(int *param_1,uint param_2,HPALETTE param_3,HWND param_4)

{
  uint *this;
  short sVar1;
  UINT UVar2;
  undefined4 *puVar3;
  int *piVar4;
  uint *puVar5;
  undefined4 uVar6;
  void *pvVar7;
  int iVar8;
  bool bVar9;
  int local_90;
  UINT local_8c;
  uint local_88;
  uint local_84;
  undefined1 local_80 [4];
  undefined4 local_7c;
  undefined4 local_78;
  undefined4 local_74;
  undefined4 local_70;
  undefined4 local_6c [7];
  int local_50;
  undefined4 local_4c [7];
  int local_30;
  undefined4 local_2c [7];
  int local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006322cc;
  local_c = ExceptionList;
  pvVar7 = (void *)0x0;
  local_8c = 0;
  if (param_2 < 0x11) {
    if (param_2 == 0x10) {
      ExceptionList = &local_c;
      if ((HDC)param_1[0x58] != (HDC)0x0) {
        ExceptionList = &local_c;
        DeleteDC((HDC)param_1[0x58]);
        param_1[0x58] = 0;
      }
      piVar4 = param_1 + 0x59;
      iVar8 = 5;
      do {
        DeleteObject((HGDIOBJ)*piVar4);
        *piVar4 = 0;
        piVar4 = piVar4 + 1;
        iVar8 = iVar8 + -1;
      } while (iVar8 != 0);
      UVar2 = FUN_00606650(param_1,0x10,param_3,param_4);
      ExceptionList = local_c;
      return UVar2;
    }
    if (param_2 == 7) {
      ExceptionList = &local_c;
      SetFocus(*(HWND *)(param_1[0x52] + 0x18));
      ExceptionList = local_c;
      return local_8c;
    }
LAB_0046e6be:
    ExceptionList = &local_c;
    local_8c = FUN_00606650(param_1,param_2,param_3,param_4);
  }
  else {
    if (param_2 == 0x407) {
      ExceptionList = &local_c;
      FUN_004ece30(&local_90);
      local_4 = 2;
      if (((short)param_3 == 0xcb) && (this = (uint *)(param_1 + 0x5f), param_1[0x5f] != local_90))
      {
        piVar4 = FUN_004f2d10(*(int *)(param_1[0x54] + 0x9c),this);
        FUN_00504c40(local_2c,piVar4,3);
        local_4._0_1_ = 3;
        FUN_00513120((int)local_2c);
        if (local_10 != 0) {
          local_74 = 0x90;
          local_70 = 0x98;
          if ((*this >> 0x18 < 0x90) || (0x97 < *this >> 0x18)) {
            bVar9 = false;
          }
          else {
            bVar9 = true;
          }
          FUN_00619730();
          if (bVar9) {
            puVar5 = &local_84;
            pvVar7 = (void *)FUN_0052bed0((int)local_2c);
            puVar5 = FUN_004025b0(pvVar7,puVar5);
            local_4._0_1_ = 4;
            FUN_004f26d0(this,puVar5);
            local_4._0_1_ = 3;
            FUN_00619730();
          }
          else {
            local_7c = 8;
            local_78 = 0x10;
            if ((*this >> 0x18 < 8) || (0xf < *this >> 0x18)) {
              bVar9 = false;
            }
            else {
              bVar9 = true;
            }
            FUN_00619730();
            if (bVar9) {
              FUN_00502db0(local_6c,piVar4,3);
              local_4 = CONCAT31(local_4._1_3_,5);
              bVar9 = false;
              FUN_00513120((int)local_6c);
              while ((local_50 != 0 && (!bVar9))) {
                uVar6 = FUN_0052bed0((int)local_6c);
                FUN_00504c40(local_4c,uVar6,3);
                local_4._0_1_ = 6;
                FUN_00513120((int)local_4c);
                bVar9 = local_30 != 0;
                if (bVar9) {
                  puVar5 = &local_88;
                  pvVar7 = (void *)FUN_0052bed0((int)local_4c);
                  puVar5 = FUN_004025b0(pvVar7,puVar5);
                  local_4._0_1_ = 7;
                  FUN_004f26d0(this,puVar5);
                  local_4._0_1_ = 6;
                  FUN_00619730();
                }
                local_4 = CONCAT31(local_4._1_3_,5);
                FUN_00504d40(local_4c);
                FUN_005130d0((int)local_6c);
              }
              local_4._1_3_ = (undefined3)((uint)local_4 >> 8);
              local_4._0_1_ = 3;
              FUN_00502eb0(local_6c);
            }
          }
        }
        FUN_00429440((void *)param_1[0x54],this);
        (**(code **)(*param_1 + 0x30))();
        local_4 = CONCAT31(local_4._1_3_,2);
        FUN_00504d40(local_2c);
      }
    }
    else {
      if (param_2 != 0x408) goto LAB_0046e6be;
      if ((short)param_3 != 0xcb) {
        return 0;
      }
      iVar8 = param_1[0x52];
      ExceptionList = &local_c;
      sVar1 = FUN_005f3040(iVar8 + 0x98);
      if (sVar1 != 0) {
        iVar8 = FUN_00609650((void *)param_1[0x53],iVar8 + 0x98,0,0);
        pvVar7 = (void *)FUN_0060a860((void *)param_1[0x5e],iVar8);
      }
      if (pvVar7 == (void *)0x0) {
        FUN_004ece30(&local_90);
        local_4 = 1;
        FUN_004f26d0(param_1 + 0x5f,&local_90);
      }
      else {
        puVar3 = FUN_0042d170(pvVar7,local_80);
        local_4 = 0;
        FUN_004f26d0(param_1 + 0x5f,puVar3);
      }
    }
    local_4 = 0xffffffff;
    FUN_00619730();
  }
  ExceptionList = local_c;
  return local_8c;
}

