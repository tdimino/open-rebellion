
void __fastcall FUN_00609a00(void *param_1)

{
  int *piVar1;
  int iVar2;
  uint uVar3;
  char cVar4;
  undefined1 uVar5;
  
  (**(code **)(**(int **)((int)param_1 + 0xa0) + 8))();
  cVar4 = '\0';
  for (piVar1 = (int *)(**(code **)(**(int **)((int)param_1 + 0xa0) + 8))(); piVar1 != (int *)0x0;
      piVar1 = (int *)(**(code **)(*piVar1 + 0xc))()) {
    if (cVar4 != '\0') goto LAB_00609a3e;
    if (piVar1[8] != 0) {
      cVar4 = *(char *)(piVar1[8] + 8);
    }
  }
  if (cVar4 != '\0') {
LAB_00609a3e:
    *(char *)((int)param_1 + 0xd4) = cVar4;
  }
  uVar5 = *(undefined1 *)((int)param_1 + 0xd4);
  iVar2 = FUN_005fc0f0(*(int **)((int)param_1 + 0xa4));
  uVar3 = FUN_005fc0e0(*(int **)((int)param_1 + 0xa4));
  FUN_005fcfe0(*(void **)((int)param_1 + 0xa4),0,0,uVar3,iVar2,uVar5);
  if (*(int **)((int)param_1 + 0x110) != (int *)0x0) {
    uVar5 = *(undefined1 *)((int)param_1 + 0xd4);
    iVar2 = FUN_005fc0f0(*(int **)((int)param_1 + 0x110));
    uVar3 = FUN_005fc0e0(*(int **)((int)param_1 + 0x110));
    FUN_005fcfe0(*(void **)((int)param_1 + 0x110),0,0,uVar3,iVar2,uVar5);
  }
  for (piVar1 = (int *)(**(code **)(**(int **)((int)param_1 + 0xa0) + 8))(); piVar1 != (int *)0x0;
      piVar1 = (int *)(**(code **)(*piVar1 + 0xc))()) {
    FUN_00609960(param_1,(int)piVar1);
  }
  if (*(HWND *)((int)param_1 + 0x18) != (HWND)0x0) {
    InvalidateRect(*(HWND *)((int)param_1 + 0x18),(RECT *)0x0,0);
  }
  return;
}

