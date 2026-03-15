
undefined4 __fastcall FUN_00606ab0(void *param_1)

{
  int iVar1;
  undefined4 uVar2;
  
  SelectObject(*(HDC *)((int)param_1 + 0xd0),*(HGDIOBJ *)((int)param_1 + 0xd8));
  SelectObject(*(HDC *)((int)param_1 + 0xd0),*(HGDIOBJ *)((int)param_1 + 0xd4));
  SelectObject(*(HDC *)((int)param_1 + 0xd0),*(HGDIOBJ *)((int)param_1 + 0xdc));
  DeleteDC(*(HDC *)((int)param_1 + 0xd0));
  *(undefined4 *)((int)param_1 + 0xd0) = 0;
  *(undefined4 *)((int)param_1 + 0xd4) = 0;
  *(undefined4 *)((int)param_1 + 0xd8) = 0;
  *(undefined4 *)((int)param_1 + 0xdc) = 0;
  uVar2 = 0;
  iVar1 = FUN_006073f0(param_1);
  if (iVar1 != 0) {
    uVar2 = *(undefined4 *)((int)param_1 + 0x108);
  }
  *(undefined4 *)((int)param_1 + 0x108) = 0;
  return uVar2;
}

