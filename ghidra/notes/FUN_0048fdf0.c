
void __thiscall FUN_0048fdf0(int param_1,int param_2,uint *param_3,int param_4)

{
  bool bVar1;
  ushort uVar2;
  int *piVar3;
  int iVar4;
  uint uVar5;
  int *piVar6;
  int *piVar7;
  undefined1 *puVar8;
  
  if (param_2 != 0x106) {
    return;
  }
  piVar3 = FUN_004f2d10(*(int *)(param_1 + 0x20),param_3);
  piVar7 = (int *)0x0;
  if ((*param_3 >> 0x18 < 0x30) || (0x3b < *param_3 >> 0x18)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  FUN_00619730();
  if (bVar1) {
    *(undefined4 *)(param_1 + 0x40) = 1;
    if (piVar3 == (int *)0x0) {
      *(undefined4 *)(param_1 + 0x40) = 0;
    }
    else {
      FUN_004f26d0((void *)(param_1 + 0x5c),param_3);
    }
    piVar7 = piVar3;
    if (((*(byte *)(piVar3 + 0x14) & 8) != 0) && ((char)piVar3[0x10] == '\a')) {
      *(undefined4 *)(param_1 + 0x40) = 0;
    }
  }
  if (*(int *)(param_1 + 0x40) == 0) {
    return;
  }
  *(undefined2 *)(param_1 + 0x30) = 0x474;
  if ((*(byte *)(piVar7 + 0x14) & 8) == 0) {
    if (param_4 == 0) {
      *(undefined2 *)(param_1 + 0x2e) = 0x41b;
      uVar5 = FUN_004c5000(piVar3);
      *(short *)(param_1 + 0x2c) = (short)uVar5;
      if (piVar7 == (int *)0x0) {
        piVar6 = (int *)0x0;
      }
      else {
        piVar6 = piVar7 + 0xc;
      }
      FUN_0060b9d0((void *)(param_1 + 0x44),0x7162,piVar6,(undefined4 *)0x0,(undefined4 *)0x0,
                   (undefined4 *)0x0);
      if (piVar7 == (int *)0x0) {
        piVar7 = (int *)0x0;
      }
      else {
        piVar7 = piVar7 + 0xc;
      }
      FUN_0060b9d0((void *)(param_1 + 0x50),0x7163,piVar7,(undefined4 *)0x0,(undefined4 *)0x0,
                   (undefined4 *)0x0);
      if ((*param_3 >> 0x18 < 0x30) || (0x37 < *param_3 >> 0x18)) {
        bVar1 = false;
      }
      else {
        bVar1 = true;
      }
      FUN_00619730();
      if (bVar1) {
        switch(*param_3 & 0xffffff) {
        case 0x240:
          puVar8 = &DAT_000000f7;
          break;
        case 0x241:
          puVar8 = &DAT_000000e3;
          break;
        case 0x242:
          if ((*(byte *)(piVar3 + 0x2c) & 8) == 0) {
            puVar8 = &DAT_000000cf;
          }
          else {
            puVar8 = &DAT_000000d9;
          }
          break;
        case 0x243:
          puVar8 = &DAT_000000ed;
          break;
        default:
          goto switchD_0049000d_caseD_244;
        case 0x280:
          puVar8 = &DAT_0000010b;
          break;
        case 0x281:
          puVar8 = &DAT_00000101;
        }
        uVar2 = FUN_004c4990(puVar8);
        *(ushort *)(param_1 + 0x32) = uVar2;
      }
    }
    else {
      *(undefined2 *)(param_1 + 0x2e) = 0x41a;
      if (piVar7 == (int *)0x0) {
        piVar3 = (int *)0x0;
      }
      else {
        piVar3 = piVar7 + 0xc;
      }
      FUN_0060b9d0((void *)(param_1 + 0x44),0x7160,piVar3,(undefined4 *)0x0,(undefined4 *)0x0,
                   (undefined4 *)0x0);
      if (piVar7 == (int *)0x0) {
        piVar7 = (int *)0x0;
      }
      else {
        piVar7 = piVar7 + 0xc;
      }
      FUN_0060b9d0((void *)(param_1 + 0x50),0x7161,piVar7,(undefined4 *)0x0,(undefined4 *)0x0,
                   (undefined4 *)0x0);
    }
  }
  else {
    iVar4 = FUN_004edc80((int)piVar7);
    *(ushort *)(param_1 + 0x2e) = (-(ushort)(iVar4 != 1) & 5) + 0x42e;
    if (piVar7 == (int *)0x0) {
      piVar3 = (int *)0x0;
    }
    else {
      piVar3 = piVar7 + 0xc;
    }
    FUN_0060b9d0((void *)(param_1 + 0x44),0x7164,piVar3,(undefined4 *)0x0,(undefined4 *)0x0,
                 (undefined4 *)0x0);
    if (piVar7 == (int *)0x0) {
      piVar7 = (int *)0x0;
    }
    else {
      piVar7 = piVar7 + 0xc;
    }
    FUN_0060b9d0((void *)(param_1 + 0x50),0x7365,piVar7,(undefined4 *)0x0,(undefined4 *)0x0,
                 (undefined4 *)0x0);
  }
switchD_0049000d_caseD_244:
  if ((*param_3 >> 0x18 < 0x30) || (0x37 < *param_3 >> 0x18)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  FUN_00619730();
  if (bVar1) {
    switch(*param_3 & 0xffffff) {
    case 0x240:
      *(undefined4 *)(param_1 + 0x28) = 0x17;
      return;
    case 0x241:
      *(undefined4 *)(param_1 + 0x28) = 0x18;
      return;
    case 0x242:
      *(undefined4 *)(param_1 + 0x28) = 0x16;
      return;
    case 0x243:
      *(undefined4 *)(param_1 + 0x28) = 0x19;
      return;
    case 0x280:
      *(undefined4 *)(param_1 + 0x28) = 0x1a;
      return;
    case 0x281:
      *(undefined4 *)(param_1 + 0x28) = 0x1b;
      return;
    }
  }
  else {
    *(undefined4 *)(param_1 + 0x28) = 0x15;
  }
  return;
}

