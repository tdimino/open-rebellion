
undefined4 __thiscall FUN_005b84e0(void *this,int param_1)

{
  undefined4 uVar1;
  int *piVar2;
  
  if (*(uint *)((int)this + 0xc) < *(uint *)((int)this + 0x10)) {
    *(uint *)((int)this + 0xc) = *(uint *)((int)this + 0xc) + 1;
    *(undefined4 *)(param_1 + 0x35c) = 1;
    uVar1 = *(undefined4 *)(param_1 + 0xec);
    piVar2 = (int *)FUN_005aaf90();
    (**(code **)(*piVar2 + 0x28))(0x2c,uVar1,*(undefined4 *)((int)this + 4),1);
    FUN_005c5f20(*(void **)((int)this + 0x28),param_1);
    return 1;
  }
  return 0;
}

