
void __thiscall FUN_00440f80(void *param_1,void *param_2)

{
  void *pvVar1;
  int iVar2;
  uint uVar3;
  ushort uVar4;
  void *pvStack_4;
  
  pvVar1 = param_2;
  *(uint *)((int)param_1 + 0xc0) = *(uint *)((int)param_1 + 0xc0) | 4;
  pvStack_4 = param_1;
  FUN_005f4dd0(param_2,&pvStack_4);
  uVar3 = (uint)pvStack_4 & 0xffff;
  pvStack_4 = (void *)((uint)pvStack_4 & 0xff);
  *(uint *)((int)param_1 + 0xa8) = uVar3 >> 8;
  FUN_005f4dd0(pvVar1,&param_2);
  uVar4 = 0;
  if ((ushort)param_2 != 0) {
    do {
      FUN_00441120(param_1,pvVar1);
      uVar4 = uVar4 + 1;
    } while (uVar4 < (ushort)param_2);
  }
  FUN_00422620(param_1);
  if ((short)pvStack_4 != *(short *)((int)param_1 + 200)) {
    SendMessageA(*(HWND *)((int)param_1 + 0x18),0x111,(int)pvStack_4 << 0x10 | 0x140,0);
  }
  *(undefined4 *)((int)param_1 + 0xbc) = 0;
  for (iVar2 = FUN_005f5060((int)param_1 + 0x6c); iVar2 != 0; iVar2 = *(int *)(iVar2 + 0x10)) {
    if (((byte)*(undefined4 *)(iVar2 + 0x24) & 0x3f) == 1) {
      SetWindowPos(*(HWND *)(iVar2 + 0x18),(HWND)0x1,0,0,0,0,3);
    }
  }
  if (*(int *)((int)param_1 + 0x248) != 0) {
    SetWindowPos(*(HWND *)(*(int *)((int)param_1 + 0x248) + 0x18),(HWND)0x1,0,0,0,0,3);
  }
  *(uint *)((int)param_1 + 0xc0) = *(uint *)((int)param_1 + 0xc0) & 0xfffffffb;
  FUN_0041d1f0(0);
  return;
}

