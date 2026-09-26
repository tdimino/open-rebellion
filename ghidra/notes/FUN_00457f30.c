
void __thiscall FUN_00457f30(void *this,int param_1,undefined4 param_2,uint *param_3)

{
  RECT *lpRect;
  uint *puVar1;
  int *this_00;
  void *pvVar2;
  char *pcVar3;
  undefined4 local_24 [3];
  undefined4 local_18 [3];
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  puVar1 = param_3;
  local_4 = 0xffffffff;
  puStack_8 = &LAB_006304f0;
  local_c = ExceptionList;
  lpRect = (RECT *)((int)this + param_1 * 0x38 + 0x168);
  if (*(uint *)((int)this + param_1 * 0x38 + 0x180) != *param_3) {
    param_1 = CONCAT22(DAT_0065d424,0x1839);
    ExceptionList = &local_c;
    FUN_005f2fc0(local_24,&param_1);
    local_4 = 0;
    this_00 = FUN_004f2d10(*(int *)(*(int *)((int)this + 0x14c) + 0x9c),puVar1);
    if (this_00 != (int *)0x0) {
      pvVar2 = FUN_004f6270(this_00,local_18);
      local_4._0_1_ = 1;
      FUN_005f30d0(local_24,(int)pvVar2);
      local_4 = (uint)local_4._1_3_ << 8;
      FUN_005f2ff0(local_18);
    }
    pvVar2 = (void *)lpRect[2].bottom;
    FUN_004f26d0(&lpRect[1].right,puVar1);
    pcVar3 = (char *)FUN_00583c40((int)local_24);
    FUN_00601aa0(pvVar2,pcVar3);
    InvalidateRect(*(HWND *)((int)this + 0x18),lpRect,0);
    local_4 = 0xffffffff;
    FUN_005f2ff0(local_24);
  }
  ExceptionList = local_c;
  return;
}

