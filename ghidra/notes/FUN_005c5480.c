
void __fastcall FUN_005c5480(void *param_1)

{
  undefined4 uVar1;
  
  if (*(int *)((int)param_1 + 0x11c) == 0) {
    *(undefined4 *)((int)param_1 + 0x11c) = 1;
    uVar1 = *(undefined4 *)((int)param_1 + 0x114);
    *(undefined4 *)((int)param_1 + 0x114) = *(undefined4 *)((int)param_1 + 0x94);
    *(undefined4 *)((int)param_1 + 0x94) = uVar1;
    uVar1 = *(undefined4 *)((int)param_1 + 0x118);
    *(undefined4 *)((int)param_1 + 0x118) = *(undefined4 *)((int)param_1 + 0x98);
    *(undefined4 *)((int)param_1 + 0x98) = uVar1;
    FUN_006030c0(param_1,0x28);
  }
  return;
}

