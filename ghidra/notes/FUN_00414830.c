
void __fastcall FUN_00414830(void *param_1)

{
  int iVar1;
  HDC hdc;
  LPBYTE pBVar2;
  undefined4 uVar3;
  undefined4 uStack_20;
  undefined4 uStack_1c;
  undefined4 uStack_18;
  int iStack_14;
  undefined4 uStack_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_0062aa58;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  if (*(int **)((int)param_1 + 0xc0) != (int *)0x0) {
    ExceptionList = &pvStack_c;
    (**(code **)(**(int **)((int)param_1 + 0xc0) + 8))();
    iVar1 = *(int *)((int)param_1 + 0xc0);
    if (*(int *)((int)param_1 + 0xb8) == iVar1) {
      *(undefined4 *)((int)param_1 + 0xb8) = 0;
    }
    if (*(int *)((int)param_1 + 0xbc) == iVar1) {
      *(undefined4 *)((int)param_1 + 0xbc) = 0;
    }
    PostMessageA(*(HWND *)((int)param_1 + 0x18),0x469,0,iVar1);
    *(undefined4 *)((int)param_1 + 0xc0) = 0;
  }
  iVar1 = ShowCursor(0);
  while (-1 < iVar1) {
    iVar1 = ShowCursor(0);
  }
  if (*(int **)((int)param_1 + 0xc4) != (int *)0x0) {
    (**(code **)(**(int **)((int)param_1 + 0xc4) + 8))();
    DestroyWindow(*(HWND *)(*(int *)((int)param_1 + 0xc4) + 0x18));
    if (*(undefined4 **)((int)param_1 + 0xc4) != (undefined4 *)0x0) {
      (**(code **)**(undefined4 **)((int)param_1 + 0xc4))(1);
    }
    *(undefined4 *)((int)param_1 + 0xc4) = 0;
  }
  hdc = GetDC(*(HWND *)((int)param_1 + 0x18));
  PatBlt(hdc,0,0,*(int *)((int)param_1 + 0x38),*(int *)((int)param_1 + 0x3c),0x42);
  FUN_00600230(hdc);
  ReleaseDC(*(HWND *)((int)param_1 + 0x18),hdc);
  switch(*(undefined4 *)((int)param_1 + 0xdc)) {
  case 1:
    uVar3 = 0x65;
    break;
  case 2:
    uVar3 = 0x66;
    break;
  default:
    goto switchD_0041492c_caseD_3;
  case 7:
    *(undefined4 *)((int)param_1 + 0x130) = 1;
    *(undefined4 *)((int)param_1 + 0x134) = 0;
    pBVar2 = FUN_00401440();
    FUN_005fba30((char *)pBVar2,s_textstra_dll_006a7ecc,2);
    if (pBVar2 != (LPBYTE)0x0) {
      FUN_00618b60(pBVar2);
    }
    thunk_FUN_00409280();
    FUN_0051c740(&uStack_20);
    uStack_18 = *(undefined4 *)((int)param_1 + 0xec);
    uStack_1c = *(undefined4 *)((int)param_1 + 0xe8);
    uStack_10 = *(undefined4 *)((int)param_1 + 0xf0);
    iStack_14 = *(int *)((int)param_1 + 0x104);
    DAT_006b1208 = (uint)(iStack_14 == 7);
    uStack_4 = 0;
    FUN_00401b90(&uStack_20,*(undefined4 *)((int)param_1 + 0xe4),*(int *)((int)param_1 + 0x108));
    FUN_004146e0(param_1,(*(int *)((int)param_1 + 0x108) != 1) + 0x68,0,0);
    uStack_4 = 0xffffffff;
    FUN_0051c780(&uStack_20);
    ExceptionList = pvStack_c;
    return;
  case 0x10:
    uVar3 = 0x6b;
    break;
  case 0x11:
    uVar3 = 0x6a;
    break;
  case 0x12:
    uVar3 = 0x6d;
    break;
  case 0x13:
    uVar3 = 0x6c;
  }
  FUN_004146e0(param_1,uVar3,0,0);
switchD_0041492c_caseD_3:
  ExceptionList = pvStack_c;
  return;
}

