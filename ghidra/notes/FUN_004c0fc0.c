
void __thiscall FUN_004c0fc0(void *this,undefined4 param_1)

{
  bool bVar1;
  int *piVar2;
  uint *puVar3;
  void *pvVar4;
  void *pvVar5;
  undefined4 *puVar6;
  undefined2 local_48;
  undefined2 local_46;
  uint local_44;
  undefined2 local_40;
  undefined2 local_3e;
  uint local_3c;
  undefined2 local_38;
  undefined2 local_36;
  undefined2 local_34;
  undefined2 local_32;
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
  puStack_8 = &LAB_0063a435;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  if ((DAT_006b2924 & 1) == 0) {
    DAT_006b2924 = DAT_006b2924 | 1;
    ExceptionList = &local_c;
    FUN_005f52c0((undefined4 *)&DAT_006b2918);
    FUN_00618c20(0x4c1540);
  }
  switch(param_1) {
  case 1:
    FUN_0041d8c0(0x11,0);
    break;
  case 2:
  case 0x12:
    FUN_005f5540(&DAT_006b2918);
    pvVar5 = (void *)FUN_00618b70(0x1c);
    local_4 = 0;
    if (pvVar5 == (void *)0x0) {
      pvVar5 = (void *)0x0;
    }
    else {
      pvVar5 = FUN_005f55d0(pvVar5,0x109);
    }
    local_4 = 0xffffffff;
    FUN_005f5440(&DAT_006b2918,pvVar5);
    puVar6 = &param_1;
    param_1 = CONCAT22(DAT_0065d424,0x1624);
    goto LAB_004c13ff;
  case 3:
    FUN_005f5540(&DAT_006b2918);
    pvVar5 = (void *)FUN_00618b70(0x1c);
    local_4 = 1;
    if (pvVar5 == (void *)0x0) {
      pvVar5 = (void *)0x0;
    }
    else {
      pvVar5 = FUN_005f55d0(pvVar5,0x121);
    }
    local_4 = 0xffffffff;
    FUN_005f5440(&DAT_006b2918,pvVar5);
    puVar6 = (undefined4 *)&local_48;
    local_48 = 0x1622;
    local_46 = DAT_0065d424;
    goto LAB_004c1474;
  case 4:
    FUN_005f5540(&DAT_006b2918);
  case 0x11:
    FUN_0041d8c0(0x17,0);
    break;
  case 5:
    FUN_0041d8c0(0x20,0);
    break;
  case 6:
    FUN_0041d8c0(0x91,0);
    break;
  case 7:
    FUN_005f5540(&DAT_006b2918);
    local_44 = 0x34000280;
    local_4 = 2;
    piVar2 = FUN_004f2d10(*(int *)((int)this + 0x14),&local_44);
    local_4 = 0xffffffff;
    FUN_00619730();
    if (piVar2 != (int *)0x0) {
      for (pvVar5 = (void *)piVar2[7]; pvVar5 != (void *)0x0;
          pvVar5 = *(void **)((int)pvVar5 + 0x1c)) {
        puVar3 = FUN_004025b0(pvVar5,&local_30);
        local_1c = 0x90;
        local_18 = 0x98;
        local_4 = 3;
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
        local_4 = 4;
        local_20 = pvVar4;
        if (pvVar4 == (void *)0x0) {
          pvVar5 = (void *)0x0;
        }
        else {
          puVar3 = FUN_004025b0(pvVar5,&local_2c);
          local_4 = CONCAT31(local_4._1_3_,5);
          pvVar5 = FUN_005f55d0(pvVar4,*puVar3 & 0xffffff);
        }
        local_4 = 4;
        if (pvVar4 != (void *)0x0) {
          FUN_00619730();
        }
        local_4 = 0xffffffff;
        FUN_005f5440(&DAT_006b2918,pvVar5);
      }
    }
    local_40 = 0x1627;
    local_3e = DAT_0065d424;
    puVar6 = (undefined4 *)&local_40;
    goto LAB_004c13ff;
  case 8:
  case 0x14:
    FUN_005f5540(&DAT_006b2918);
    local_20 = (void *)FUN_00618b70(0x1c);
    local_4 = 10;
    if (local_20 == (void *)0x0) {
      pvVar5 = (void *)0x0;
    }
    else {
      pvVar5 = FUN_005f55d0(local_20,0x121);
    }
    local_4 = 0xffffffff;
    FUN_005f5440(&DAT_006b2918,pvVar5);
    local_34 = 0x1622;
    local_32 = DAT_0065d424;
    puVar6 = (undefined4 *)&local_34;
LAB_004c1474:
    FUN_0041d890(0x93,0x6b2918,puVar6);
    break;
  case 9:
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
  case 0xe:
    FUN_0041d8c0(0x13,0);
    break;
  case 0xf:
    FUN_0041d8c0(0x15,0);
    break;
  case 0x10:
    FUN_0041d8c0(0x14,0);
    break;
  case 0x13:
    FUN_005f5540(&DAT_006b2918);
    local_3c = 0x35000281;
    local_4 = 6;
    piVar2 = FUN_004f2d10(*(int *)((int)this + 0x14),&local_3c);
    local_4 = 0xffffffff;
    FUN_00619730();
    if (piVar2 != (int *)0x0) {
      for (pvVar5 = (void *)piVar2[7]; pvVar5 != (void *)0x0;
          pvVar5 = *(void **)((int)pvVar5 + 0x1c)) {
        puVar3 = FUN_004025b0(pvVar5,&local_28);
        local_14 = 0x90;
        local_10 = 0x98;
        local_4 = 7;
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
        local_4 = 8;
        local_20 = pvVar4;
        if (pvVar4 == (void *)0x0) {
          pvVar5 = (void *)0x0;
        }
        else {
          puVar3 = FUN_004025b0(pvVar5,&local_24);
          local_4 = CONCAT31(local_4._1_3_,9);
          pvVar5 = FUN_005f55d0(pvVar4,*puVar3 & 0xffffff);
        }
        local_4 = 8;
        if (pvVar4 != (void *)0x0) {
          FUN_00619730();
        }
        local_4 = 0xffffffff;
        FUN_005f5440(&DAT_006b2918,pvVar5);
      }
    }
    local_38 = 0x1628;
    local_36 = DAT_0065d424;
    puVar6 = (undefined4 *)&local_38;
LAB_004c13ff:
    FUN_0041d890(0x92,0x6b2918,puVar6);
  }
  ExceptionList = local_c;
  return;
}

