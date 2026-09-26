
void __thiscall FUN_004ef750(void *this,int *param_1,void *param_2)

{
  int iVar1;
  uint uVar2;
  
  iVar1 = FUN_005774f0(this,param_1,param_2);
  if (iVar1 != 0) {
    uVar2 = DAT_006b9068;
    if ((*(byte *)((int)this + 0xac) & 4) == 0) {
      uVar2 = DAT_006b9064;
    }
    FUN_00540230((void *)(*(int *)((int)this + 0x54) + 0x18),uVar2);
    FUN_00540230((void *)(*(int *)((int)this + 0x54) + 0x28),DAT_006b90ac);
    FUN_00540270((void *)(*(int *)((int)this + 0x54) + 0x28),DAT_006b9074);
    FUN_00540230((void *)(*(int *)((int)this + 0x54) + 0x20),DAT_006b907c);
    FUN_00540270((void *)(*(int *)((int)this + 0x54) + 0x20),DAT_006b90c0);
    FUN_004ef070(this,param_2);
  }
  return;
}

