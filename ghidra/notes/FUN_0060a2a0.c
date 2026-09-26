
void __thiscall FUN_0060a2a0(void *this,int *param_1,int param_2)

{
  int iVar1;
  uint uVar2;
  LONG LVar3;
  HDC hdc;
  int iVar4;
  LPCSTR lpchText;
  tagRECT local_20;
  int local_10;
  undefined4 local_c;
  int local_4;
  
  iVar1 = *(int *)(param_2 + 0x48);
  local_20.left = *(int *)(param_2 + 0x40);
  local_20.top = *(int *)(param_2 + 0x44);
  local_20.right = *(int *)(param_2 + 0x48);
  local_10 = *(int *)(param_2 + 0x40);
  local_20.bottom = *(int *)(param_2 + 0x4c);
  local_4 = *(int *)(param_2 + 0x4c);
  local_c = *(undefined4 *)(param_2 + 0x44);
  hdc = GetDC(*(HWND *)((int)this + 0x18));
  FUN_006002c0(this,hdc);
  iVar4 = FUN_00583c40(param_2 + 0x14);
  if (iVar4 != 0) {
    lpchText = (LPCSTR)FUN_00583c40(param_2 + 0x14);
    DrawTextA(hdc,lpchText,-1,&local_20,*(uint *)((int)this + 0xe4) & 0xfffffff4 | 0x400);
    LVar3 = local_20.bottom;
    uVar2 = *(uint *)((int)this + 0xe4);
    if ((uVar2 & 8) != 0) {
      local_20.bottom = local_4;
      local_20.top = local_20.top + (local_4 - LVar3);
    }
    if ((uVar2 & 2) == 0) {
      if ((uVar2 & 1) != 0) {
        local_20.right = local_20.right - local_20.left;
        local_20.left = (iVar1 - local_10) / 2 - local_20.right / 2;
        local_20.right = local_20.right + local_20.left;
      }
    }
    else {
      local_20.left = local_20.left + (iVar1 - local_20.right);
      local_20.right = iVar1;
    }
    local_20.right = local_20.right + *(int *)((int)this + 0xe8);
    local_20.left = local_20.left + *(int *)((int)this + 0xe8);
    local_20.top = local_20.top + *(int *)((int)this + 0xec);
    local_20.bottom = local_20.bottom + *(int *)((int)this + 0xec);
  }
  FUN_006002f0(this,hdc);
  ReleaseDC(*(HWND *)((int)this + 0x18),hdc);
  *param_1 = local_20.left;
  param_1[1] = local_20.top;
  param_1[2] = local_20.right;
  param_1[3] = local_20.bottom;
  return;
}

