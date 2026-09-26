
void __thiscall FUN_0046c8a0(void *this,int param_1)

{
  void *pvVar1;
  int *piVar2;
  
  if (*(int *)((int)this + 0x118) != param_1) {
    if (param_1 == 0x97) {
      FUN_006075e0(this,0x32);
      pvVar1 = (void *)((int)this + 0x6c);
      piVar2 = (int *)FUN_00604500(pvVar1,0x68);
      (**(code **)(*piVar2 + 4))(5);
      (**(code **)(**(int **)((int)this + 0x130) + 8))();
      (**(code **)(**(int **)((int)this + 0x138) + 8))();
      piVar2 = (int *)FUN_00604500(pvVar1,0xca);
      (**(code **)(*piVar2 + 8))();
      piVar2 = (int *)FUN_00604500(pvVar1,0xcb);
      (**(code **)(*piVar2 + 8))();
      (**(code **)(**(int **)((int)this + 0x128) + 4))(5);
      SetFocus(*(HWND *)(*(int *)((int)this + 0x128) + 0x18));
      *(undefined4 *)((int)this + 0x118) = 0x97;
    }
    else if (param_1 == 0x98) {
      FUN_006075e0(this,0x33);
      (**(code **)(**(int **)((int)this + 0x128) + 8))();
      pvVar1 = (void *)((int)this + 0x6c);
      piVar2 = (int *)FUN_00604500(pvVar1,0x68);
      (**(code **)(*piVar2 + 8))();
      (**(code **)(**(int **)((int)this + 0x130) + 4))(5);
      (**(code **)(**(int **)((int)this + 0x138) + 4))(5);
      piVar2 = (int *)FUN_00604500(pvVar1,0xca);
      (**(code **)(*piVar2 + 4))(5);
      piVar2 = (int *)FUN_00604500(pvVar1,0xcb);
      (**(code **)(*piVar2 + 4))(5);
      SetFocus(*(HWND *)(*(int *)((int)this + 0x130) + 0x18));
      *(undefined4 *)((int)this + 0x118) = 0x98;
      return;
    }
  }
  return;
}

