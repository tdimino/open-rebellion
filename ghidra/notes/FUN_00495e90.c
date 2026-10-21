
void __thiscall FUN_00495e90(void *param_1,int param_2,uint *param_3,uint *param_4)

{
  int *piVar1;
  int *piVar2;
  
  if (param_2 == 0x340) {
    piVar1 = FUN_004f2d10(*(int *)((int)param_1 + 0x20),param_3);
    piVar2 = FUN_004f3220(*(int *)((int)param_1 + 0x20),param_4);
    FUN_004f26d0((void *)((int)param_1 + 0x5c),param_4);
    if ((piVar1 != (int *)0x0) && (piVar2 != (int *)0x0)) {
      if (*(int *)((int)param_1 + 0x68) == 0) {
        *(undefined4 *)((int)param_1 + 0x40) = 1;
        *(undefined4 *)((int)param_1 + 0x68) = 1;
        FUN_0060b9d0((void *)((int)param_1 + 0x44),0x70a8,piVar1 + 0xc,piVar2 + 0xc,
                     (undefined4 *)0x0,(undefined4 *)0x0);
        FUN_0060b9d0((void *)((int)param_1 + 0x50),0x70a9,piVar1 + 0xc,piVar2 + 0xc,
                     (undefined4 *)0x0,(undefined4 *)0x0);
        *(undefined4 *)((int)param_1 + 0x28) = 0x10;
        if (*(int *)((int)param_1 + 0x20) == 1) {
          *(undefined2 *)((int)param_1 + 0x2e) = 0x407;
          *(undefined2 *)((int)param_1 + 0x30) = 0x460;
        }
        else {
          *(undefined2 *)((int)param_1 + 0x2e) = 0x406;
          *(undefined2 *)((int)param_1 + 0x30) = 0x45f;
        }
      }
      if (*(int *)((int)param_1 + 0x40) != 0) {
        FUN_0060b9d0((void *)((int)param_1 + 0x50),0x70aa,piVar1 + 0xc,piVar2 + 0xc,
                     (undefined4 *)0x0,(undefined4 *)0x0);
      }
    }
    return;
  }
  FUN_004c5020(param_1,param_2,param_3);
  return;
}

