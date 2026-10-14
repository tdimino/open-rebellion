
void __thiscall FUN_004981c0(int param_1,int param_2,uint *param_3)

{
  int iVar1;
  bool bVar2;
  void *pvVar3;
  int *piVar4;
  uint *puVar5;
  undefined4 *puVar6;
  uint uVar7;
  void *this;
  void *pvStack_78;
  uint uStack_74;
  void *pvStack_70;
  uint uStack_6c;
  uint uStack_68;
  undefined4 uStack_64;
  undefined4 uStack_60;
  undefined4 uStack_5c;
  undefined4 uStack_58;
  undefined4 uStack_54;
  undefined4 uStack_50;
  undefined4 uStack_4c;
  undefined4 uStack_48;
  undefined4 uStack_44;
  undefined4 uStack_40;
  undefined4 uStack_3c;
  undefined4 uStack_38;
  undefined4 uStack_34;
  undefined4 uStack_30;
  undefined4 uStack_2c;
  undefined4 uStack_28;
  undefined4 uStack_24;
  undefined4 uStack_20;
  undefined4 uStack_1c;
  undefined4 uStack_18;
  undefined4 uStack_14;
  undefined4 uStack_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_00636658;
  pvStack_c = ExceptionList;
  if (param_2 != 0x105) {
    return;
  }
  ExceptionList = &pvStack_c;
  piVar4 = FUN_004f2d10(*(int *)(param_1 + 0x20),param_3);
  pvStack_78 = (void *)0x0;
  uStack_68 = 0;
  if (piVar4 == (int *)0x0) {
    ExceptionList = pvStack_c;
    return;
  }
  this = (void *)piVar4[7];
  FUN_004f26d0((void *)(param_1 + 0x5c),param_3);
  do {
    if (this == (void *)0x0) break;
    puVar5 = FUN_004025b0(this,&uStack_74);
    uStack_64 = 0x90;
    uStack_60 = 0x98;
    uStack_4 = 0;
    if ((*puVar5 >> 0x18 < 0x90) || (0x97 < *puVar5 >> 0x18)) {
      bVar2 = false;
    }
    else {
      bVar2 = true;
    }
    FUN_00619730();
    uStack_4 = 0xffffffff;
    FUN_00619730();
    pvVar3 = this;
    if (!bVar2) {
      this = *(void **)((int)this + 0x1c);
      pvVar3 = pvStack_78;
    }
    pvStack_78 = pvVar3;
  } while (pvStack_78 == (void *)0x0);
  pvStack_70 = (void *)piVar4[7];
  if (*(int *)(param_1 + 0x68) != 0) goto LAB_00498526;
  iVar1 = *(int *)(param_1 + 0x20);
  *(undefined4 *)(param_1 + 0x40) = 1;
  *(undefined4 *)(param_1 + 0x68) = 1;
  if (iVar1 == 1) {
    if (((byte)piVar4[9] & 0xc0) != 0x40) goto LAB_004982f2;
  }
  else if ((iVar1 != 2) || (((byte)piVar4[9] & 0xc0) != 0x80)) {
LAB_004982f2:
    *(undefined4 *)(param_1 + 0x40) = 0;
  }
  uStack_5c = 8;
  uStack_58 = 0x10;
  *(ushort *)(param_1 + 0x30) = (iVar1 != 1) + 0x45b;
  if ((*param_3 >> 0x18 < 8) || (0xf < *param_3 >> 0x18)) {
    bVar2 = false;
  }
  else {
    bVar2 = true;
  }
  FUN_00619730();
  if (bVar2) {
    *(undefined2 *)(param_1 + 0x30) = 0x45a;
    *(undefined4 *)(param_1 + 100) = 1;
LAB_00498350:
    *(undefined4 *)(param_1 + 0x34) = 0x80;
  }
  else {
    uStack_54 = 0x20;
    uStack_50 = 0x22;
    if ((*param_3 >> 0x18 < 0x20) || (0x21 < *param_3 >> 0x18)) {
      bVar2 = false;
    }
    else {
      bVar2 = true;
    }
    FUN_00619730();
    if (bVar2) {
      *(undefined2 *)(param_1 + 0x30) = 0x45d;
      *(undefined4 *)(param_1 + 100) = 4;
      *(undefined4 *)(param_1 + 0x34) = 0x40;
      goto LAB_00498357;
    }
    uStack_4c = 0x14;
    uStack_48 = 0x1c;
    if ((*param_3 >> 0x18 < 0x14) || (0x1b < *param_3 >> 0x18)) {
      bVar2 = false;
    }
    else {
      bVar2 = true;
    }
    FUN_00619730();
    if (bVar2) {
      *(undefined2 *)(param_1 + 0x30) = 0x45a;
      *(undefined4 *)(param_1 + 100) = 2;
      goto LAB_00498350;
    }
    uStack_44 = 0x30;
    uStack_40 = 0x40;
    if ((*param_3 >> 0x18 < 0x30) || (0x3f < *param_3 >> 0x18)) {
      bVar2 = false;
    }
    else {
      bVar2 = true;
    }
    FUN_00619730();
    if (!bVar2) {
      *(undefined4 *)(param_1 + 100) = 3;
      uStack_3c = 0x22;
      uStack_38 = 0x28;
      if ((*param_3 >> 0x18 < 0x22) || (0x27 < *param_3 >> 0x18)) {
        bVar2 = false;
      }
      else {
        bVar2 = true;
      }
      FUN_00619730();
      if (bVar2) {
        *(undefined4 *)(param_1 + 0x34) = 0x40;
      }
      else {
        uStack_34 = 0x28;
        uStack_30 = 0x2c;
        if ((*param_3 >> 0x18 < 0x28) || (0x2b < *param_3 >> 0x18)) {
          bVar2 = false;
        }
        else {
          bVar2 = true;
        }
        FUN_00619730();
        if (bVar2) {
          *(undefined4 *)(param_1 + 0x34) = 8;
        }
        else {
          uStack_2c = 0x2c;
          uStack_28 = 0x30;
          if ((*param_3 >> 0x18 < 0x2c) || (0x2f < *param_3 >> 0x18)) {
            bVar2 = false;
          }
          else {
            bVar2 = true;
          }
          FUN_00619730();
          if (bVar2) {
            *(undefined4 *)(param_1 + 0x34) = 8;
          }
          else {
            uStack_1c = 0x10;
            uStack_18 = 0x14;
            uVar7 = *param_3 >> 0x18;
            bVar2 = true;
            uStack_4 = 1;
            if ((uVar7 < 0x10) || (0x13 < uVar7)) {
              uStack_24 = 0x1c;
              uStack_20 = 0x20;
              if ((uVar7 < 0x1c) || (0x1f < uVar7)) {
                bVar2 = false;
              }
              FUN_00619730();
              if (bVar2) goto LAB_0049876f;
              bVar2 = false;
            }
            else {
LAB_0049876f:
              bVar2 = true;
            }
            uStack_4 = 0xffffffff;
            FUN_00619730();
            if (bVar2) {
              puVar5 = FUN_004025b0(pvStack_70,&uStack_6c);
              uStack_14 = 0x90;
              uStack_10 = 0x98;
              uStack_4 = 2;
              if ((*puVar5 >> 0x18 < 0x90) || (0x97 < *puVar5 >> 0x18)) {
                bVar2 = false;
              }
              else {
                bVar2 = true;
              }
              FUN_00619730();
              uStack_4 = 0xffffffff;
              FUN_00619730();
              if (!bVar2) goto LAB_00498350;
              *(undefined4 *)(param_1 + 0x34) = 0x40;
            }
          }
        }
      }
    }
  }
LAB_00498357:
  if (pvStack_78 == (void *)0x0) {
    *(undefined4 *)(param_1 + 0x40) = 0;
    uVar7 = uStack_68;
  }
  else {
    uVar7 = *(uint *)((int)pvStack_78 + 0x88) >> 0xc & 1;
  }
  if (uVar7 != 0) {
    *(undefined4 *)(param_1 + 0x40) = 0;
  }
  if (*(int *)(param_1 + 100) == 0) {
    *(undefined4 *)(param_1 + 0x40) = 0;
  }
  if (*(int *)(param_1 + 0x40) == 0) goto LAB_00498526;
  switch(*(int *)(param_1 + 100)) {
  case 1:
    if (pvStack_78 == (void *)0x0) {
      puVar6 = (undefined4 *)0x0;
    }
    else {
      puVar6 = (undefined4 *)((int)pvStack_78 + 0x30);
    }
    FUN_0060b9d0((void *)(param_1 + 0x44),0x7078,piVar4 + 0xc,puVar6,(undefined4 *)0x0,
                 (undefined4 *)0x0);
    if (pvStack_78 == (void *)0x0) {
      puVar6 = (undefined4 *)0x0;
      uVar7 = 0x7079;
    }
    else {
      puVar6 = (undefined4 *)((int)pvStack_78 + 0x30);
      uVar7 = 0x7079;
    }
    break;
  case 2:
    if (pvStack_78 == (void *)0x0) {
      puVar6 = (undefined4 *)0x0;
    }
    else {
      puVar6 = (undefined4 *)((int)pvStack_78 + 0x30);
    }
    FUN_0060b9d0((void *)(param_1 + 0x44),0x7076,piVar4 + 0xc,puVar6,(undefined4 *)0x0,
                 (undefined4 *)0x0);
    if (pvStack_78 == (void *)0x0) {
      puVar6 = (undefined4 *)0x0;
    }
    else {
      puVar6 = (undefined4 *)((int)pvStack_78 + 0x30);
    }
    uVar7 = 0x7077;
    break;
  case 3:
    if (pvStack_78 == (void *)0x0) {
      puVar6 = (undefined4 *)0x0;
    }
    else {
      puVar6 = (undefined4 *)((int)pvStack_78 + 0x30);
    }
    FUN_0060b9d0((void *)(param_1 + 0x44),0x707a,piVar4 + 0xc,puVar6,(undefined4 *)0x0,
                 (undefined4 *)0x0);
    if (pvStack_78 == (void *)0x0) {
      puVar6 = (undefined4 *)0x0;
    }
    else {
      puVar6 = (undefined4 *)((int)pvStack_78 + 0x30);
    }
    FUN_0060b9d0((void *)(param_1 + 0x50),0x707b,piVar4 + 0xc,puVar6,(undefined4 *)0x0,
                 (undefined4 *)0x0);
    *(undefined4 *)(param_1 + 0x28) = 6;
    if (*(int *)(param_1 + 0x20) == 1) {
      *(undefined2 *)(param_1 + 0x2e) = 0x3fc;
    }
    else {
      *(undefined2 *)(param_1 + 0x2e) = 0x3fd;
    }
    goto LAB_00498526;
  case 4:
    if (pvStack_78 == (void *)0x0) {
      puVar6 = (undefined4 *)0x0;
    }
    else {
      puVar6 = (undefined4 *)((int)pvStack_78 + 0x30);
    }
    FUN_0060b9d0((void *)(param_1 + 0x44),0x707c,piVar4 + 0xc,puVar6,(undefined4 *)0x0,
                 (undefined4 *)0x0);
    if (pvStack_78 == (void *)0x0) {
      puVar6 = (undefined4 *)0x0;
    }
    else {
      puVar6 = (undefined4 *)((int)pvStack_78 + 0x30);
    }
    FUN_0060b9d0((void *)(param_1 + 0x50),0x707d,piVar4 + 0xc,puVar6,(undefined4 *)0x0,
                 (undefined4 *)0x0);
    *(undefined4 *)(param_1 + 0x28) = 6;
    if (*(int *)(param_1 + 0x20) == 1) {
      *(undefined2 *)(param_1 + 0x2e) = 0x3fe;
      goto LAB_00498526;
    }
  default:
    *(undefined4 *)(param_1 + 0x40) = 0;
    goto LAB_00498526;
  }
  FUN_0060b9d0((void *)(param_1 + 0x50),uVar7,piVar4 + 0xc,puVar6,(undefined4 *)0x0,
               (undefined4 *)0x0);
  *(undefined4 *)(param_1 + 0x28) = 5;
  if (*(int *)(param_1 + 0x20) == 1) {
    *(undefined2 *)(param_1 + 0x2e) = 0x3fa;
  }
  else {
    *(undefined2 *)(param_1 + 0x2e) = 0x3fb;
  }
LAB_00498526:
  if ((*(int *)(param_1 + 0x40) != 0) &&
     ((*(int *)(param_1 + 100) == 2 || (*(int *)(param_1 + 100) == 3)))) {
    if (pvStack_78 == (void *)0x0) {
      puVar6 = (undefined4 *)0x0;
    }
    else {
      puVar6 = (undefined4 *)((int)pvStack_78 + 0x30);
    }
    FUN_0060b9d0((void *)(param_1 + 0x50),0x707e,piVar4 + 0xc,puVar6,(undefined4 *)0x0,
                 (undefined4 *)0x0);
  }
  ExceptionList = pvStack_c;
  return;
}

