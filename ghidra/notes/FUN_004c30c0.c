
void __thiscall FUN_004c30c0(void *this,undefined4 param_1)

{
  bool bVar1;
  int *piVar2;
  uint *puVar3;
  void *pvVar4;
  void *pvVar5;
  undefined4 *puVar6;
  uint local_54;
  undefined2 local_50;
  undefined2 local_4e;
  undefined2 local_4c;
  undefined2 local_4a;
  uint local_48;
  undefined2 local_44;
  undefined2 local_42;
  uint local_40;
  undefined2 local_3c;
  undefined2 local_3a;
  undefined2 local_38;
  undefined2 local_36;
  uint local_34;
  uint local_30;
  uint local_2c;
  uint local_28;
  uint local_24;
  void *local_20;
  undefined4 local_1c;
  undefined4 local_18;
  undefined4 local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0063a5ff;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  if ((DAT_006b2934 & 1) == 0) {
    DAT_006b2934 = DAT_006b2934 | 1;
    ExceptionList = &local_c;
    FUN_005f52c0((undefined4 *)&DAT_006b2928);
    FUN_00618c20(0x4c3740);
  }
  switch(param_1) {
  case 1:
    FUN_0041d8c0(0x11,0);
    break;
  case 2:
  case 0xe:
    FUN_005f5540(&DAT_006b2928);
    FUN_0041d8c0(0x13,0);
    break;
  case 3:
    FUN_005f5540(&DAT_006b2928);
    local_54 = 0x20000005;
    local_4 = 1;
    piVar2 = FUN_004f2d10(*(int *)((int)this + 0x14),&local_54);
    local_4 = 0xffffffff;
    FUN_00619730();
    if (piVar2 == (int *)0x0) goto switchD_004c311d_caseD_9;
    pvVar5 = (void *)piVar2[7];
    pvVar4 = (void *)FUN_00618b70(0x1c);
    local_4 = 2;
    local_20 = pvVar4;
    if (pvVar4 == (void *)0x0) {
      pvVar5 = (void *)0x0;
    }
    else {
      puVar3 = FUN_004025b0(pvVar5,&local_34);
      local_4 = CONCAT31(local_4._1_3_,3);
      pvVar5 = FUN_005f55d0(pvVar4,*puVar3 & 0xffffff);
    }
    local_4 = 2;
    if (pvVar4 != (void *)0x0) {
      FUN_00619730();
    }
    local_4 = 0xffffffff;
    FUN_005f5440(&DAT_006b2928,pvVar5);
    local_50 = 0x1623;
    local_4e = DAT_0065d424;
    puVar6 = (undefined4 *)&local_50;
    goto LAB_004c35fa;
  case 4:
    FUN_005f5540(&DAT_006b2928);
    local_20 = (void *)FUN_00618b70(0x1c);
    local_4 = 4;
    if (local_20 == (void *)0x0) {
      pvVar5 = (void *)0x0;
    }
    else {
      pvVar5 = FUN_005f55d0(local_20,0x109);
    }
    local_4 = 0xffffffff;
    FUN_005f5440(&DAT_006b2928,pvVar5);
    puVar6 = (undefined4 *)&local_4c;
    local_4c = 0x1624;
    local_4a = DAT_0065d424;
    goto LAB_004c366f;
  case 5:
    FUN_005f5540(&DAT_006b2928);
    FUN_0041d8c0(0x20,0);
    break;
  case 6:
    FUN_0041d8c0(0x91,0);
    break;
  case 7:
    FUN_005f5540(&DAT_006b2928);
    local_48 = 0x30000240;
    local_4 = 5;
    piVar2 = FUN_004f2d10(*(int *)((int)this + 0x14),&local_48);
    local_4 = 0xffffffff;
    FUN_00619730();
    if (piVar2 != (int *)0x0) {
      for (pvVar5 = (void *)piVar2[7]; pvVar5 != (void *)0x0;
          pvVar5 = *(void **)((int)pvVar5 + 0x1c)) {
        puVar3 = FUN_004025b0(pvVar5,&local_30);
        local_1c = 0x90;
        local_18 = 0x98;
        local_4 = 6;
        if ((*puVar3 >> 0x18 < 0x90) || (0x97 < *puVar3 >> 0x18)) {
          bVar1 = false;
        }
        else {
          bVar1 = true;
        }
        FUN_00619730();
        local_4 = 0xffffffff;
        FUN_00619730();
        if (bVar1) break;
      }
      if (pvVar5 != (void *)0x0) {
        pvVar4 = (void *)FUN_00618b70(0x1c);
        local_4 = 7;
        local_20 = pvVar4;
        if (pvVar4 == (void *)0x0) {
          pvVar5 = (void *)0x0;
        }
        else {
          puVar3 = FUN_004025b0(pvVar5,&local_2c);
          local_4 = CONCAT31(local_4._1_3_,8);
          pvVar5 = FUN_005f55d0(pvVar4,*puVar3 & 0xffffff);
        }
        local_4 = 7;
        if (pvVar4 != (void *)0x0) {
          FUN_00619730();
        }
        local_4 = 0xffffffff;
        FUN_005f5440(&DAT_006b2928,pvVar5);
      }
    }
    local_44 = 0x1625;
    local_42 = DAT_0065d424;
    puVar6 = (undefined4 *)&local_44;
    goto LAB_004c35fa;
  case 8:
  case 0x14:
    FUN_005f5540(&DAT_006b2928);
    local_20 = (void *)FUN_00618b70(0x1c);
    local_4 = 0xd;
    if (local_20 == (void *)0x0) {
      pvVar5 = (void *)0x0;
    }
    else {
      pvVar5 = FUN_005f55d0(local_20,0x109);
    }
    local_4 = 0xffffffff;
    FUN_005f5440(&DAT_006b2928,pvVar5);
    puVar6 = (undefined4 *)&local_38;
    local_38 = 0x1624;
    local_36 = DAT_0065d424;
LAB_004c366f:
    FUN_0041d890(0x92,0x6b2928,puVar6);
    break;
  case 9:
switchD_004c311d_caseD_9:
    FUN_0041d8c0(0x80,0);
    break;
  case 0xb:
    FUN_0041d8c0(0x11,0);
    FUN_0041dbe0();
    break;
  case 0xc:
    FUN_0041d9d0();
    break;
  case 0xd:
    thunk_FUN_0042d760();
    if ((DAT_006b28b0 & 0x8000) == 0) {
      FUN_00439f20((int)this);
      FUN_0041d770(1,0x82);
    }
    FUN_0041da80();
    break;
  case 0xf:
    FUN_005f5540(&DAT_006b2928);
    FUN_0041d8c0(0x15,0);
    break;
  case 0x10:
    FUN_005f5540(&DAT_006b2928);
    FUN_0041d8c0(0x16,0);
    break;
  case 0x11:
    FUN_005f5540(&DAT_006b2928);
    FUN_0041d8c0(0x17,0);
    break;
  case 0x12:
    FUN_005f5540(&DAT_006b2928);
    pvVar5 = (void *)FUN_00618b70(0x1c);
    local_4 = 0;
    if (pvVar5 == (void *)0x0) {
      pvVar5 = (void *)0x0;
    }
    else {
      pvVar5 = FUN_005f55d0(pvVar5,0x121);
    }
    local_4 = 0xffffffff;
    FUN_005f5440(&DAT_006b2928,pvVar5);
    param_1 = CONCAT22(DAT_0065d424,0x1622);
    puVar6 = &param_1;
    goto LAB_004c35fa;
  case 0x13:
    FUN_005f5540(&DAT_006b2928);
    local_40 = 0x32000242;
    local_4 = 9;
    piVar2 = FUN_004f2d10(*(int *)((int)this + 0x14),&local_40);
    local_4 = 0xffffffff;
    FUN_00619730();
    if (piVar2 != (int *)0x0) {
      for (pvVar5 = (void *)piVar2[7]; pvVar5 != (void *)0x0;
          pvVar5 = *(void **)((int)pvVar5 + 0x1c)) {
        puVar3 = FUN_004025b0(pvVar5,&local_28);
        local_14 = 0x90;
        local_10 = 0x98;
        local_4 = 10;
        if ((*puVar3 >> 0x18 < 0x90) || (0x97 < *puVar3 >> 0x18)) {
          bVar1 = false;
        }
        else {
          bVar1 = true;
        }
        FUN_00619730();
        local_4 = 0xffffffff;
        FUN_00619730();
        if (bVar1) break;
      }
      if (pvVar5 != (void *)0x0) {
        pvVar4 = (void *)FUN_00618b70(0x1c);
        local_4 = 0xb;
        local_20 = pvVar4;
        if (pvVar4 == (void *)0x0) {
          pvVar5 = (void *)0x0;
        }
        else {
          puVar3 = FUN_004025b0(pvVar5,&local_24);
          local_4 = CONCAT31(local_4._1_3_,0xc);
          pvVar5 = FUN_005f55d0(pvVar4,*puVar3 & 0xffffff);
        }
        local_4 = 0xb;
        if (pvVar4 != (void *)0x0) {
          FUN_00619730();
        }
        local_4 = 0xffffffff;
        FUN_005f5440(&DAT_006b2928,pvVar5);
      }
    }
    local_3c = 0x1626;
    local_3a = DAT_0065d424;
    puVar6 = (undefined4 *)&local_3c;
LAB_004c35fa:
    FUN_0041d890(0x93,0x6b2928,puVar6);
  }
  ExceptionList = local_c;
  return;
}

