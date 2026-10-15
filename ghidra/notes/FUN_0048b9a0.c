
void __thiscall FUN_0048b9a0(int param_1,int param_2,uint *param_3,uint *param_4)

{
  bool bVar1;
  int *piVar2;
  int *piVar3;
  uint *puVar4;
  int *piVar5;
  uint uVar6;
  int *piVar7;
  uint uStack_1c;
  uint uStack_18;
  undefined4 uStack_14;
  undefined4 uStack_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_00635220;
  pvStack_c = ExceptionList;
  if (param_2 == 0x305) {
    ExceptionList = &pvStack_c;
    piVar2 = FUN_004f2d10(*(int *)(param_1 + 0x20),param_3);
    piVar3 = FUN_004f2d10(*(int *)(param_1 + 0x20),param_4);
    for (piVar7 = piVar3; piVar7 != (int *)0x0; piVar7 = (int *)piVar7[7]) {
      puVar4 = FUN_004025b0(piVar7,&uStack_1c);
      uStack_14 = 0x90;
      uStack_10 = 0x98;
      uStack_4 = 0;
      if ((*puVar4 >> 0x18 < 0x90) || (0x97 < *puVar4 >> 0x18)) {
        bVar1 = false;
      }
      else {
        bVar1 = true;
      }
      FUN_00619730();
      uStack_4 = 0xffffffff;
      FUN_00619730();
      if (bVar1) break;
    }
    if (piVar7 != (int *)0x0) {
      puVar4 = FUN_004025b0(piVar7,&uStack_18);
      uStack_4 = 1;
      FUN_004f26d0((void *)(param_1 + 0x5c),puVar4);
      uStack_4 = 0xffffffff;
      FUN_00619730();
    }
    if ((piVar2 != (int *)0x0) && (piVar3 != (int *)0x0)) {
      if (*(int *)(param_1 + 100) == 0) {
        *(undefined4 *)(param_1 + 100) = 1;
        if (piVar7 == (int *)0x0) {
          piVar5 = (int *)0x0;
        }
        else {
          piVar5 = piVar7 + 0xc;
        }
        FUN_0060b9d0((void *)(param_1 + 0x44),0x7083,piVar2 + 0xc,piVar3 + 0xc,piVar5,
                     (undefined4 *)0x0);
        if (piVar7 == (int *)0x0) {
          piVar7 = (int *)0x0;
        }
        else {
          piVar7 = piVar7 + 0xc;
        }
        FUN_0060b9d0((void *)(param_1 + 0x50),0x7084,piVar2 + 0xc,piVar3 + 0xc,piVar7,
                     (undefined4 *)0x0);
        *(undefined4 *)(param_1 + 0x28) = 0xc;
        *(undefined2 *)(param_1 + 0x30) = 0x456;
        *(undefined4 *)(param_1 + 0x34) = 0x100;
      }
      FUN_0060b9d0((void *)(param_1 + 0x50),0x7085,piVar2 + 0xc,piVar3 + 0xc,(undefined4 *)0x0,
                   (undefined4 *)0x0);
      *(undefined4 *)(param_1 + 0x40) = 1;
      uVar6 = (uint)piVar2[9] >> 6 & 3;
      if (uVar6 == 1) {
        if (*(int *)(param_1 + 0x20) != 2) {
          ExceptionList = pvStack_c;
          return;
        }
      }
      else if ((uVar6 == 2) && (*(int *)(param_1 + 0x20) != 1)) {
        ExceptionList = pvStack_c;
        return;
      }
      *(undefined4 *)(param_1 + 0x40) = 0;
    }
  }
  ExceptionList = pvStack_c;
  return;
}

