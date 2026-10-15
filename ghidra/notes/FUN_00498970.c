
void __thiscall FUN_00498970(int param_1,int param_2,uint *param_3,uint *param_4)

{
  bool bVar1;
  int *piVar2;
  int *piVar3;
  int *piVar4;
  uint *puVar5;
  uint uVar6;
  uint uStack_18;
  undefined4 uStack_14;
  undefined4 uStack_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_006366e0;
  pvStack_c = ExceptionList;
  if (param_2 == 0x304) {
    ExceptionList = &pvStack_c;
    piVar3 = FUN_004f2d10(*(int *)(param_1 + 0x20),param_3);
    piVar4 = FUN_004f2d10(*(int *)(param_1 + 0x20),param_4);
    param_4 = (uint *)piVar4;
    for (; piVar4 != (int *)0x0; piVar4 = (int *)piVar4[7]) {
      puVar5 = FUN_004025b0(piVar4,(uint *)&param_3);
      uStack_14 = 0x90;
      uStack_10 = 0x98;
      uStack_4 = 0;
      if ((*puVar5 >> 0x18 < 0x90) || (0x97 < *puVar5 >> 0x18)) {
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
    piVar2 = (int *)param_4;
    if (piVar4 != (int *)0x0) {
      puVar5 = FUN_004025b0(piVar4,&uStack_18);
      uStack_4 = 1;
      FUN_004f26d0((void *)(param_1 + 0x5c),puVar5);
      uStack_4 = 0xffffffff;
      FUN_00619730();
    }
    if ((piVar3 != (int *)0x0) && (piVar2 != (int *)0x0)) {
      if (*(int *)(param_1 + 100) == 0) {
        *(undefined4 *)(param_1 + 100) = 1;
        FUN_0060b9d0((void *)(param_1 + 0x44),0x7080,(undefined4 *)0x0,(undefined4 *)0x0,
                     (undefined4 *)0x0,(undefined4 *)0x0);
        if (piVar4 == (int *)0x0) {
          piVar4 = (int *)0x0;
        }
        else {
          piVar4 = piVar4 + 0xc;
        }
        FUN_0060b9d0((void *)(param_1 + 0x50),0x7081,piVar3 + 0xc,piVar2 + 0xc,piVar4,
                     (undefined4 *)0x0);
        *(undefined4 *)(param_1 + 0x28) = 0xc;
        *(undefined2 *)(param_1 + 0x30) = 0x456;
      }
      FUN_0060b9d0((void *)(param_1 + 0x50),0x7082,piVar3 + 0xc,piVar2 + 0xc,(undefined4 *)0x0,
                   (undefined4 *)0x0);
      *(undefined4 *)(param_1 + 0x40) = 1;
      uVar6 = (uint)piVar3[9] >> 6 & 3;
      if (uVar6 == 1) {
        if (*(int *)(param_1 + 0x20) != 2) {
          ExceptionList = pvStack_c;
          return;
        }
      }
      else {
        if (uVar6 != 2) {
          ExceptionList = pvStack_c;
          return;
        }
        if (*(int *)(param_1 + 0x20) != 1) {
          ExceptionList = pvStack_c;
          return;
        }
      }
      *(undefined4 *)(param_1 + 0x40) = 0;
    }
  }
  ExceptionList = pvStack_c;
  return;
}

