
int __fastcall FUN_005f8a30(int *param_1)

{
  uint uVar1;
  int iVar2;
  int *piVar3;
  uint uVar4;
  uint uVar5;
  uint *puVar6;
  uint *puVar7;
  bool bVar8;
  
  if (param_1[0x42] != 0) {
    puVar7 = (uint *)(param_1 + 0xd);
    puVar6 = puVar7;
    for (iVar2 = 0xf; iVar2 != 0; iVar2 = iVar2 + -1) {
      *puVar6 = 0xffffffff;
      puVar6 = puVar6 + 1;
    }
    uVar5 = param_1[0x25];
    param_1[0x24] = 0;
    param_1[0x25] = 0;
    (**(code **)(*(int *)param_1[0x42] + 0x30))((int *)param_1[0x42],&DAT_006be380,&LAB_005f89c0,0);
    uVar1 = param_1[0x24];
    if (uVar1 != 0) {
      uVar5 = 1;
      do {
        uVar4 = uVar5;
        puVar6 = puVar7;
        if (uVar5 < uVar1) {
          do {
            puVar6 = puVar6 + 1;
            uVar1 = *puVar7;
            if (*puVar6 < uVar1) {
              *puVar7 = *puVar6;
              *puVar6 = uVar1;
            }
            uVar4 = uVar4 + 1;
          } while (uVar4 < (uint)param_1[0x24]);
        }
        uVar1 = param_1[0x24];
        puVar7 = puVar7 + 1;
        bVar8 = uVar5 < uVar1;
        uVar5 = uVar5 + 1;
      } while (bVar8);
      uVar5 = 0;
    }
    param_1[0x43] = -1;
    uVar1 = 0;
    if (DAT_006ac294 != 0) {
      piVar3 = param_1 + 0xd;
      do {
        if (*piVar3 == param_1[0x44]) {
          param_1[0x43] = uVar1;
          break;
        }
        uVar1 = uVar1 + 1;
        piVar3 = piVar3 + 1;
      } while (uVar1 < DAT_006ac294);
    }
    param_1[0x1c] = 0;
    param_1[0x25] = param_1[0x24];
    FUN_005f9030(param_1);
    if (param_1[0x25] != uVar5) {
      iVar2 = FUN_005f88c0((int)param_1);
      DAT_006be39c = iVar2 << 0x1c;
      (**(code **)(param_1[6] + 4))();
      if (((uint)param_1[0x25] < uVar5) && (param_1[0x25] == 1)) {
        FUN_005f8600(param_1);
        FUN_005ae1e0();
        PostMessageA((HWND)param_1[0xb],0x40c,4,0);
      }
    }
  }
  return param_1[0x24];
}

