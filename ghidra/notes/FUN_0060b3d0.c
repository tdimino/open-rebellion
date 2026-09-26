
undefined4 __thiscall FUN_0060b3d0(void *this,int param_1)

{
  int iVar1;
  int unaff_ESI;
  int unaff_EDI;
  undefined4 uVar2;
  undefined1 auStack_30 [8];
  int iStack_28;
  int aiStack_24 [5];
  tagRECT local_10;
  
  uVar2 = 0;
  iVar1 = FUN_0060b660(this,param_1);
  if (iVar1 != 0) {
    SetRect(&local_10,0,0,*(int *)((int)this + 0x30),*(int *)((int)this + 0x34));
    (**(code **)(*(int *)this + 0x30))(auStack_30);
    (**(code **)(*(int *)this + 0x2c))(aiStack_24);
    InvalidateRect(*(HWND *)((int)this + 0x18),(RECT *)0x0,0);
    if (((*(uint **)((int)this + 0xc4) != (uint *)0x0) && (*(int *)((int)this + 0xcc) != 0)) &&
       (*(int *)((int)this + 0xd0) != 0)) {
      if (*(void **)((int)this + 200) != (void *)0x0) {
        FUN_005fcc30(*(void **)((int)this + 200),*(uint **)((int)this + 0xc4),0,0,0,0,
                     (undefined4 *)0x0,0);
      }
      FUN_005fcc30(*(void **)((int)this + 0xcc),*(uint **)((int)this + 0xc4),unaff_EDI,unaff_ESI,0,0
                   ,(undefined4 *)0x0,0);
      FUN_005fcc30(*(void **)((int)this + 0xd0),*(uint **)((int)this + 0xc4),iStack_28,aiStack_24[0]
                   ,0,0,(undefined4 *)0x0,0);
      uVar2 = 1;
    }
  }
  return uVar2;
}

