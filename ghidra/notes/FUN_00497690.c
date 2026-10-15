
void __thiscall FUN_00497690(int param_1,int param_2,uint *param_3,uint *param_4)

{
  bool bVar1;
  int *piVar2;
  int *piVar3;
  
  if (param_2 == 0x303) {
    piVar2 = FUN_004f2d10(*(int *)(param_1 + 0x20),param_3);
    piVar3 = FUN_004f3220(*(int *)(param_1 + 0x20),param_4);
    if (((piVar2 != (int *)0x0) && (piVar3 != (int *)0x0)) && (*(int *)(param_1 + 0x68) == 0)) {
      *(undefined4 *)(param_1 + 0x68) = 1;
      if ((*param_3 >> 0x18 < 0x20) || (0x2f < *param_3 >> 0x18)) {
        bVar1 = false;
      }
      else {
        bVar1 = true;
      }
      FUN_00619730();
      if (bVar1) {
        *(undefined4 *)(param_1 + 0x40) = 1;
        *(undefined4 *)(param_1 + 100) = 5;
      }
      if (*(int *)(param_1 + 100) == 0) {
        *(undefined4 *)(param_1 + 0x40) = 0;
      }
      if (*(int *)(param_1 + 0x40) != 0) {
        FUN_0060b9d0((void *)(param_1 + 0x44),0x70b0,piVar2 + 0xc,piVar3 + 0xc,(undefined4 *)0x0,
                     (undefined4 *)0x0);
        FUN_0060b9d0((void *)(param_1 + 0x50),0x70b1,piVar2 + 0xc,piVar3 + 0xc,(undefined4 *)0x0,
                     (undefined4 *)0x0);
        *(ushort *)(param_1 + 0x2e) = (*(int *)(param_1 + 0x20) != 1) + 0x3f7;
        *(undefined4 *)(param_1 + 0x28) = 3;
        FUN_004f26d0((void *)(param_1 + 0x5c),param_4);
      }
    }
  }
  return;
}

