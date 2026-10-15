
void __thiscall FUN_00491060(int param_1,int param_2,uint *param_3,int param_4)

{
  int *piVar1;
  uint uVar2;
  
  if (param_2 != 0x153) {
    return;
  }
  piVar1 = FUN_004f3220(*(int *)(param_1 + 0x20),param_3);
  FUN_004f26d0((void *)(param_1 + 0x5c),param_3);
  if (piVar1 == (int *)0x0) {
    return;
  }
  if (param_4 == 0) {
    return;
  }
  uVar2 = (uint)piVar1[9] >> 6 & 3;
  if (uVar2 == 1) {
    if (*(int *)(param_1 + 0x20) != 2) goto LAB_004910c9;
  }
  else if (uVar2 == 2) {
    if (*(int *)(param_1 + 0x20) == 1) {
      *(undefined4 *)(param_1 + 0x40) = 1;
    }
    goto LAB_004910c9;
  }
  *(undefined4 *)(param_1 + 0x40) = 1;
LAB_004910c9:
  FUN_0060b9d0((void *)(param_1 + 0x44),0x7150,piVar1 + 0xc,(undefined4 *)0x0,(undefined4 *)0x0,
               (undefined4 *)0x0);
  FUN_0060b9d0((void *)(param_1 + 0x50),0x7151,piVar1 + 0xc,(undefined4 *)0x0,(undefined4 *)0x0,
               (undefined4 *)0x0);
  *(undefined2 *)(param_1 + 0x2e) = 1000;
  *(undefined4 *)(param_1 + 0x28) = 0x1c;
  *(undefined2 *)(param_1 + 0x30) = 0x44c;
  return;
}

