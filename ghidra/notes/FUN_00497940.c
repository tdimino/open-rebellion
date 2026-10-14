
void __thiscall FUN_00497940(int param_1,int param_2,uint *param_3)

{
  int iVar1;
  bool bVar2;
  void *pvVar3;
  int *this;
  uint *puVar4;
  int *piVar5;
  undefined4 *puVar6;
  uint uVar7;
  void *pvVar8;
  void *pvStack_84;
  uint uStack_7c;
  uint uStack_78;
  uint uStack_74;
  undefined1 auStack_70 [4];
  undefined4 uStack_6c;
  undefined4 uStack_68;
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
  puStack_8 = &LAB_006365d0;
  pvStack_c = ExceptionList;
  if (param_2 != 0x107) {
    return;
  }
  ExceptionList = &pvStack_c;
  this = FUN_004f2d10(*(int *)(param_1 + 0x20),param_3);
  pvStack_84 = (void *)0x0;
  if (this == (int *)0x0) {
    ExceptionList = pvStack_c;
    return;
  }
  FUN_004f26d0((void *)(param_1 + 0x5c),param_3);
  pvVar8 = (void *)this[7];
  do {
    if (pvVar8 == (void *)0x0) break;
    puVar4 = FUN_004025b0(pvVar8,&uStack_7c);
    uStack_6c = 0x90;
    uStack_68 = 0x98;
    uStack_4 = 0;
    if ((*puVar4 >> 0x18 < 0x90) || (0x97 < *puVar4 >> 0x18)) {
      bVar2 = false;
    }
    else {
      bVar2 = true;
    }
    FUN_00619730();
    uStack_4 = 0xffffffff;
    FUN_00619730();
    pvVar3 = pvVar8;
    if (!bVar2) {
      pvVar8 = *(void **)((int)pvVar8 + 0x1c);
      pvVar3 = pvStack_84;
    }
    pvStack_84 = pvVar3;
  } while (pvStack_84 == (void *)0x0);
  pvVar8 = (void *)this[7];
  if (*(int *)(param_1 + 0x68) != 0) {
    ExceptionList = pvStack_c;
    return;
  }
  bVar2 = true;
  *(undefined4 *)(param_1 + 0x40) = 1;
  *(undefined4 *)(param_1 + 0x68) = 1;
  if (*(int *)(param_1 + 0x20) == 1) {
    if (((byte)this[9] & 0xc0) != 0x40) goto LAB_00497a69;
  }
  else if ((*(int *)(param_1 + 0x20) != 2) || (((byte)this[9] & 0xc0) != 0x80)) {
LAB_00497a69:
    *(undefined4 *)(param_1 + 0x40) = 0;
  }
  uStack_64 = 0x18;
  uStack_60 = 0x1c;
  if ((*param_3 >> 0x18 < 0x18) || (0x1b < *param_3 >> 0x18)) {
    bVar2 = false;
  }
  FUN_00619730();
  if (bVar2) {
    *(undefined4 *)(param_1 + 100) = 4;
    *(undefined4 *)(param_1 + 0x34) = 0x80;
  }
  else {
    uStack_5c = 0x14;
    uStack_58 = 0x1c;
    if ((*param_3 >> 0x18 < 0x14) || (0x1b < *param_3 >> 0x18)) {
      bVar2 = false;
    }
    else {
      bVar2 = true;
    }
    FUN_00619730();
    if (bVar2) {
      *(undefined4 *)(param_1 + 0x34) = 0x80;
      *(undefined4 *)(param_1 + 100) = 3;
    }
    else {
      uStack_54 = 0x1c;
      uStack_50 = 0x20;
      if ((*param_3 >> 0x18 < 0x1c) || (0x1f < *param_3 >> 0x18)) {
        bVar2 = false;
      }
      else {
        bVar2 = true;
      }
      FUN_00619730();
      if (bVar2) {
        bVar2 = true;
        *(undefined4 *)(param_1 + 100) = 1;
        puVar4 = FUN_004025b0(pvVar8,&uStack_78);
        uStack_4c = 0x90;
        uStack_48 = 0x98;
        uStack_4 = 1;
        if ((*puVar4 >> 0x18 < 0x90) || (0x97 < *puVar4 >> 0x18)) {
          bVar2 = false;
        }
        FUN_00619730();
        uStack_4 = 0xffffffff;
        FUN_00619730();
        if (bVar2) {
          *(undefined4 *)(param_1 + 0x34) = 0x40;
          goto LAB_00497e2a;
        }
      }
      else {
        uStack_44 = 0x10;
        uStack_40 = 0x14;
        if ((*param_3 >> 0x18 < 0x10) || (0x13 < *param_3 >> 0x18)) {
          bVar2 = false;
        }
        else {
          bVar2 = true;
        }
        FUN_00619730();
        if (!bVar2) {
          uStack_34 = 0x20;
          uStack_30 = 0x30;
          if ((*param_3 >> 0x18 < 0x20) || (0x2f < *param_3 >> 0x18)) {
            bVar2 = false;
          }
          else {
            bVar2 = true;
          }
          FUN_00619730();
          if (bVar2) {
            uStack_2c = 0x22;
            uStack_28 = 0x28;
            if ((*param_3 >> 0x18 < 0x22) || (0x27 < *param_3 >> 0x18)) {
              bVar2 = false;
            }
            else {
              bVar2 = true;
            }
            FUN_00619730();
            if (bVar2) {
              *(undefined4 *)(param_1 + 0x34) = 0x40;
              *(undefined4 *)(param_1 + 100) = 5;
            }
            else {
              uStack_24 = 0x28;
              uStack_20 = 0x2c;
              if ((*param_3 >> 0x18 < 0x28) || (0x2b < *param_3 >> 0x18)) {
                bVar2 = false;
              }
              else {
                bVar2 = true;
              }
              FUN_00619730();
              if (bVar2) {
                *(undefined4 *)(param_1 + 0x34) = 8;
                *(undefined4 *)(param_1 + 100) = 5;
              }
              else {
                uStack_1c = 0x2c;
                uStack_18 = 0x30;
                if ((*param_3 >> 0x18 < 0x2c) || (0x2f < *param_3 >> 0x18)) {
                  bVar2 = false;
                }
                else {
                  bVar2 = true;
                }
                FUN_00619730();
                if (bVar2) {
                  *(undefined4 *)(param_1 + 0x34) = 4;
                }
                *(undefined4 *)(param_1 + 100) = 5;
              }
            }
          }
          else {
            uStack_14 = 0x3c;
            uStack_10 = 0x40;
            if ((*param_3 >> 0x18 < 0x3c) || (0x3f < *param_3 >> 0x18)) {
              bVar2 = false;
            }
            else {
              bVar2 = true;
            }
            FUN_00619730();
            if (bVar2) {
              uStack_4 = 3;
              piVar5 = FUN_00402d80(this,auStack_70);
              iVar1 = *piVar5;
              FUN_00619730();
              uStack_4 = 0xffffffff;
              FUN_00619730();
              if (iVar1 == 0x3c000009) {
                *(undefined4 *)(param_1 + 0x40) = 0;
              }
              *(undefined4 *)(param_1 + 0x34) = 0x10;
              *(undefined4 *)(param_1 + 100) = 6;
            }
          }
          goto LAB_00497e2a;
        }
        *(undefined4 *)(param_1 + 100) = 2;
        puVar4 = FUN_004025b0(pvVar8,&uStack_74);
        uStack_3c = 0x90;
        uStack_38 = 0x98;
        uStack_4 = 2;
        if ((*puVar4 >> 0x18 < 0x90) || (0x97 < *puVar4 >> 0x18)) {
          bVar2 = false;
        }
        else {
          bVar2 = true;
        }
        FUN_00619730();
        uStack_4 = 0xffffffff;
        FUN_00619730();
        if (bVar2) {
          *(undefined4 *)(param_1 + 0x34) = 0x40;
          goto LAB_00497e2a;
        }
      }
      *(undefined4 *)(param_1 + 0x34) = 0x80;
    }
  }
LAB_00497e2a:
  if (pvStack_84 == (void *)0x0) {
    *(undefined4 *)(param_1 + 0x40) = 0;
  }
  if (*(int *)(param_1 + 100) == 0) {
    *(undefined4 *)(param_1 + 0x40) = 0;
  }
  if (*(int *)(param_1 + 0x40) != 0) {
    switch(*(int *)(param_1 + 100)) {
    case 1:
      if (pvStack_84 == (void *)0x0) {
        puVar6 = (undefined4 *)0x0;
      }
      else {
        puVar6 = (undefined4 *)((int)pvStack_84 + 0x30);
      }
      FUN_0060b9d0((void *)(param_1 + 0x44),0x708a,this + 0xc,puVar6,(undefined4 *)0x0,
                   (undefined4 *)0x0);
      if (pvVar8 == (void *)0x0) {
        puVar6 = (undefined4 *)0x0;
      }
      else {
        puVar6 = (undefined4 *)((int)pvVar8 + 0x30);
      }
      FUN_0060b9d0((void *)(param_1 + 0x50),0x708b,this + 0xc,puVar6,(undefined4 *)0x0,
                   (undefined4 *)0x0);
      *(ushort *)(param_1 + 0x30) = (*(int *)(param_1 + 0x20) != 1) + 0x457;
      break;
    case 2:
    case 6:
      if (pvStack_84 == (void *)0x0) {
        puVar6 = (undefined4 *)0x0;
      }
      else {
        puVar6 = (undefined4 *)((int)pvStack_84 + 0x30);
      }
      piVar5 = this + 0xc;
      FUN_0060b9d0((void *)(param_1 + 0x44),0x7088,piVar5,puVar6,(undefined4 *)0x0,(undefined4 *)0x0
                  );
      if (pvVar8 == (void *)0x0) {
        puVar6 = (undefined4 *)0x0;
      }
      else {
        puVar6 = (undefined4 *)((int)pvVar8 + 0x30);
      }
      FUN_0060b9d0((void *)(param_1 + 0x50),0x7089,piVar5,puVar6,(undefined4 *)0x0,(undefined4 *)0x0
                  );
      if (pvVar8 == (void *)0x0) {
        puVar6 = (undefined4 *)0x0;
      }
      else {
        puVar6 = (undefined4 *)((int)pvVar8 + 0x30);
      }
      FUN_0060b9d0((void *)(param_1 + 0x50),0x707f,piVar5,puVar6,(undefined4 *)0x0,(undefined4 *)0x0
                  );
      break;
    case 3:
      if (pvStack_84 == (void *)0x0) {
        puVar6 = (undefined4 *)0x0;
      }
      else {
        puVar6 = (undefined4 *)((int)pvStack_84 + 0x30);
      }
      FUN_0060b9d0((void *)(param_1 + 0x44),0x7086,this + 0xc,puVar6,(undefined4 *)0x0,
                   (undefined4 *)0x0);
      if (pvStack_84 == (void *)0x0) {
        puVar6 = (undefined4 *)0x0;
      }
      else {
        puVar6 = (undefined4 *)((int)pvStack_84 + 0x30);
      }
      FUN_0060b9d0((void *)(param_1 + 0x50),0x7087,this + 0xc,puVar6,(undefined4 *)0x0,
                   (undefined4 *)0x0);
      break;
    case 4:
      if (pvStack_84 == (void *)0x0) {
        puVar6 = (undefined4 *)0x0;
      }
      else {
        puVar6 = (undefined4 *)((int)pvStack_84 + 0x30);
      }
      FUN_0060b9d0((void *)(param_1 + 0x44),0x708e,this + 0xc,puVar6,(undefined4 *)0x0,
                   (undefined4 *)0x0);
      if (pvStack_84 == (void *)0x0) {
        puVar6 = (undefined4 *)0x0;
      }
      else {
        puVar6 = (undefined4 *)((int)pvStack_84 + 0x30);
      }
      FUN_0060b9d0((void *)(param_1 + 0x50),0x708f,this + 0xc,puVar6,(undefined4 *)0x0,
                   (undefined4 *)0x0);
      break;
    case 5:
      if (pvStack_84 == (void *)0x0) {
        puVar6 = (undefined4 *)0x0;
      }
      else {
        puVar6 = (undefined4 *)((int)pvStack_84 + 0x30);
      }
      FUN_0060b9d0((void *)(param_1 + 0x44),0x708c,this + 0xc,puVar6,(undefined4 *)0x0,
                   (undefined4 *)0x0);
      if (pvStack_84 == (void *)0x0) {
        puVar6 = (undefined4 *)0x0;
      }
      else {
        puVar6 = (undefined4 *)((int)pvStack_84 + 0x30);
      }
      FUN_0060b9d0((void *)(param_1 + 0x50),0x708d,this + 0xc,puVar6,(undefined4 *)0x0,
                   (undefined4 *)0x0);
      break;
    default:
      *(undefined4 *)(param_1 + 0x40) = 0;
    }
    uVar7 = FUN_004c5000(this);
    *(short *)(param_1 + 0x2e) = (short)uVar7;
    *(undefined4 *)(param_1 + 0x28) = 3;
  }
  ExceptionList = pvStack_c;
  return;
}

