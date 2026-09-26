
void __thiscall FUN_0042d230(void *this,int param_1,int param_2,int param_3)

{
  char *pcVar1;
  undefined4 uVar2;
  void *this_00;
  int *this_01;
  int iVar3;
  int iVar4;
  uint *puVar5;
  uint uVar6;
  int xLeft;
  undefined4 *puVar7;
  uint uVar8;
  undefined4 local_38 [3];
  RECT local_2c;
  tagRECT local_1c;
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062c7c3;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_005f2f50(local_38);
  local_4 = 0;
  if (param_1 != 0) {
    FUN_005f31f0(local_38,param_1);
    pcVar1 = (char *)FUN_00583c40((int)local_38);
    FUN_00601aa0((void *)((int)this + 0xe0),pcVar1);
    FUN_00601b80((void *)((int)this + 0xe0),*(HDC *)((int)this + 0x460));
    local_2c.left = *(LONG *)((int)this + 0xf4);
    local_2c.top = *(LONG *)((int)this + 0xf8);
    local_2c.right = *(LONG *)((int)this + 0xfc);
    local_2c.bottom = *(LONG *)((int)this + 0x100);
    InvalidateRect(*(HWND *)((int)this + 0x18),&local_2c,0);
  }
  uVar2 = FUN_006037f0(7);
  uVar6 = 0;
  if ((*(short *)((int)this + 0x114) != param_2) || (param_3 != 0)) {
    *(short *)((int)this + 0x114) = (short)param_2;
    switch(param_2) {
    case 0:
      uVar6 = (-(uint)(*(int *)((int)this + 0x9c) != 1) & 4) + 0x2d3c;
      break;
    case 1:
      uVar6 = (-(uint)(*(int *)((int)this + 0x9c) != 1) & 4) + 0x2d3d;
      break;
    case 2:
      uVar6 = (-(uint)(*(int *)((int)this + 0x9c) != 1) & 4) + 0x2d3e;
      break;
    case 3:
      uVar6 = (-(uint)(*(int *)((int)this + 0x9c) != 1) & 4) + 0x2d3f;
      break;
    case 4:
      uVar6 = (*(int *)((int)this + 0x9c) != 1) + 0x2d44;
    }
    if ((short)uVar6 != 0) {
      this_00 = (void *)FUN_00618b70(0x28);
      local_4._0_1_ = 1;
      if (this_00 == (void *)0x0) {
        this_01 = (int *)0x0;
      }
      else {
        this_01 = FUN_005fbd20(this_00,uVar2,uVar6,2);
      }
      local_4 = (uint)local_4._1_3_ << 8;
      if (*(int *)((int)this + 0x9c) == 1) {
        iVar3 = FUN_005fc0f0(this_01);
        iVar3 = iVar3 + 0x14;
        iVar4 = FUN_005fc0e0(this_01);
        iVar4 = iVar4 + 0xa7;
        xLeft = 0xa7;
      }
      else {
        iVar3 = FUN_005fc0f0(this_01);
        iVar3 = iVar3 + 0x14;
        iVar4 = FUN_005fc0e0(this_01);
        iVar4 = iVar4 + 0x231;
        xLeft = 0x231;
      }
      SetRect(&local_1c,xLeft,0x14,iVar4,iVar3);
      uVar8 = 0;
      puVar7 = (undefined4 *)0x0;
      uVar6 = 0;
      iVar3 = 0;
      puVar5 = (uint *)FUN_005ff440((int)this);
      FUN_005fcc30(this_01,puVar5,local_1c.left,local_1c.top,iVar3,uVar6,puVar7,uVar8);
      if (this_01 != (int *)0x0) {
        FUN_005fbfa0(this_01);
        FUN_00618b60((undefined *)this_01);
      }
      InvalidateRect(*(HWND *)((int)this + 0x18),&local_1c,0);
    }
  }
  uVar6 = FUN_00401ae0();
  if ((uVar6 & 0x100) != 0) {
    FUN_0042d780((int)this);
  }
  local_4 = 0xffffffff;
  FUN_005f2ff0(local_38);
  ExceptionList = local_c;
  return;
}

