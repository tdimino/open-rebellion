
undefined4 __thiscall
FUN_005a96b0(void *this,float param_1,undefined4 param_2,undefined4 param_3,void *param_4)

{
  undefined1 uVar1;
  int *piVar2;
  uint uVar3;
  undefined4 *puVar4;
  undefined4 uVar5;
  int *piVar6;
  int iVar7;
  int iVar8;
  uint uVar9;
  undefined4 *puVar10;
  uint local_20;
  undefined4 *local_1c;
  undefined **local_18 [3];
  void *pvStack_c;
  undefined1 *puStack_8;
  uint local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00652e10;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  FUN_005f52c0(local_18);
  local_18[0] = &PTR_FUN_0066c3a8;
  local_4 = 0;
  piVar2 = FUN_005ce600(param_4);
  while (piVar2 != (int *)0x0) {
    if ((float)piVar2[8] == param_1) {
      uVar3 = FUN_005ce840((int)piVar2);
      puVar4 = (undefined4 *)FUN_005ce830((int)piVar2);
      local_20 = 0;
      local_1c = (undefined4 *)FUN_00618b70(uVar3);
      if ((local_1c != (undefined4 *)0x0) && (puVar4 != (undefined4 *)0x0)) {
        puVar10 = local_1c;
        for (uVar9 = uVar3 >> 2; uVar9 != 0; uVar9 = uVar9 - 1) {
          *puVar10 = *puVar4;
          puVar4 = puVar4 + 1;
          puVar10 = puVar10 + 1;
        }
        for (uVar9 = uVar3 & 3; local_20 = uVar3, uVar9 != 0; uVar9 = uVar9 - 1) {
          *(undefined1 *)puVar10 = *(undefined1 *)puVar4;
          puVar4 = (undefined4 *)((int)puVar4 + 1);
          puVar10 = (undefined4 *)((int)puVar10 + 1);
        }
      }
      local_4 = CONCAT31(local_4._1_3_,1);
      uVar1 = (**(code **)(*piVar2 + 0x10))();
      switch(uVar1) {
      case 3:
        FUN_005b2b60((int)&local_20);
        break;
      case 9:
        FUN_005cadb0((int)&local_20);
        break;
      case 10:
        FUN_005caba0((int)&local_20);
        break;
      case 0xb:
        FUN_005cac00((int)&local_20);
        break;
      case 0xc:
        FUN_005cac30((int)&local_20);
        break;
      case 0xd:
        FUN_005b1ea0((int)&local_20);
        break;
      case 0xe:
        FUN_005b1f60((int)&local_20);
        break;
      case 0xf:
        FUN_005b2080((int)&local_20);
        break;
      case 0x10:
        FUN_005b2c70((int)&local_20);
        break;
      case 0x11:
        FUN_005b2a20((int)&local_20);
        break;
      case 0x12:
        FUN_005b2a70((int)&local_20);
        break;
      case 0x13:
        FUN_005b2ac0((int)&local_20);
        break;
      case 0x14:
        FUN_005b2b10((int)&local_20);
        break;
      case 0x16:
        FUN_005b2340((int)&local_20);
        break;
      case 0x19:
        FUN_005b8b10((int)&local_20);
        break;
      case 0x1a:
        FUN_005b8bd0((int)&local_20);
        break;
      case 0x1b:
        FUN_005b2810((int)&local_20);
        break;
      case 0x1c:
        FUN_005b27e0((int)&local_20);
        break;
      case 0x1d:
        FUN_005b27b0((int)&local_20);
        break;
      case 0x1e:
        FUN_005b28d0((int)&local_20);
        break;
      case 0x1f:
        FUN_005b28a0((int)&local_20);
        break;
      case 0x20:
        FUN_005b2870((int)&local_20);
        break;
      case 0x21:
        FUN_005b2840((int)&local_20);
        break;
      case 0x22:
        FUN_005b2900((int)&local_20);
        break;
      case 0x23:
        FUN_005b29c0((int)&local_20);
        break;
      case 0x24:
        FUN_005b2990((int)&local_20);
        break;
      case 0x25:
        FUN_005b2960((int)&local_20);
        break;
      case 0x26:
        FUN_005b2930((int)&local_20);
        break;
      case 0x27:
        FUN_005b29f0((int)&local_20);
        break;
      case 0x28:
        FUN_005b2630((int)&local_20);
        break;
      case 0x29:
        FUN_005b26a0((int)&local_20);
        break;
      case 0x2c:
        FUN_005b1e10((int)&local_20);
        break;
      case 0x2d:
        FUN_005b1e40((int)&local_20);
        break;
      case 0x2e:
        FUN_005b26d0((int)&local_20);
        break;
      case 0x2f:
        FUN_005b2720((int)&local_20);
        break;
      case 0x30:
        FUN_005cc1b0((int)&local_20);
        break;
      case 0x31:
        FUN_005c7160((int)&local_20);
        break;
      case 0x32:
        FUN_005cae40((int)&local_20);
        break;
      case 0x33:
        FUN_005cae70((int)&local_20);
        break;
      case 0x34:
        FUN_005cad20((int)&local_20);
        break;
      case 0x35:
        FUN_005cad80((int)&local_20);
        break;
      case 0x36:
        FUN_005cae10((int)&local_20);
        break;
      case 0x37:
        FUN_005caea0((int)&local_20);
        break;
      case 0x38:
        FUN_005cacf0((int)&local_20);
        break;
      case 0x39:
        FUN_005cad50((int)&local_20);
        break;
      case 0x3a:
        FUN_005cade0((int)&local_20);
        break;
      case 0x3c:
        FUN_005caf20((int)&local_20);
        break;
      case 0x3d:
        FUN_005cabd0((int)&local_20);
        break;
      case 0x3f:
        FUN_005b01d0((int)&local_20);
        break;
      case 0x40:
        FUN_005ba120((int)&local_20);
        break;
      case 0x41:
        FUN_005bdc30((int)&local_20);
        break;
      case 0x43:
        FUN_005ba5b0((int)&local_20);
        break;
      case 0x44:
        FUN_005ba520((int)&local_20);
        break;
      case 0x45:
        FUN_005ba5e0();
        break;
      case 0x4a:
        iVar7 = (**(code **)(*piVar2 + 0x14))();
        if (iVar7 < 2) {
          uVar5 = (**(code **)(*piVar2 + 0x18))();
          *(undefined4 *)((int)this + iVar7 * 4 + 0x870) = uVar5;
        }
      }
      if (piVar2 != (int *)0x0) {
        (**(code **)*piVar2)(1);
      }
      local_4 = local_4 & 0xffffff00;
      if (local_1c != (undefined4 *)0x0) {
        FUN_00618b60((undefined *)local_1c);
      }
      local_1c = (undefined4 *)0x0;
    }
    else if (param_1 <= (float)piVar2[8]) {
      for (piVar6 = (int *)thunk_FUN_005f5060((int)local_18); piVar6 != (int *)0x0;
          piVar6 = (int *)(**(code **)(*piVar6 + 8))()) {
        iVar7 = (**(code **)(*piVar6 + 0x1c))();
        iVar8 = (**(code **)(*piVar2 + 0x1c))();
        if (iVar7 == iVar8) goto LAB_005a9b34;
      }
      FUN_005f5440(local_18,piVar2);
    }
    else if (piVar2 != (int *)0x0) {
      (**(code **)*piVar2)(1);
    }
LAB_005a9b34:
    piVar2 = FUN_005ce600(param_4);
  }
  do {
    piVar2 = FUN_005ce600(local_18);
    if (piVar2 == (int *)0x0) break;
    for (piVar6 = (int *)thunk_FUN_005f5060((int)param_4); piVar6 != (int *)0x0;
        piVar6 = (int *)(**(code **)(*piVar6 + 8))()) {
      iVar7 = (**(code **)(*piVar6 + 0x1c))();
      iVar8 = (**(code **)(*piVar2 + 0x1c))();
      if (iVar7 == iVar8) goto LAB_005a9b92;
    }
    FUN_005f5440(param_4,piVar2);
LAB_005a9b92:
  } while (piVar2 != (int *)0x0);
  local_4 = 0xffffffff;
  FUN_005ce5b0(local_18);
  ExceptionList = pvStack_c;
  return 1;
}

