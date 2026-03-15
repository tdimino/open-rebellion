
uint __thiscall FUN_0051e910(void *this,int *param_1,undefined4 *param_2)

{
  bool bVar1;
  int iVar2;
  undefined3 extraout_var;
  void *pvVar4;
  uint uVar3;
  
  *param_2 = 0;
  uVar3 = (uint)(*(void **)((int)this + 0xbc) != (void *)0x0);
  if (uVar3 != 0) {
    pvVar4 = (void *)0x0;
    if (*param_1 != 2) {
      pvVar4 = FUN_00568de0(*(void **)((int)this + 0xbc),param_1);
    }
    if (pvVar4 == (void *)0x0) {
      pvVar4 = FUN_00568d70(*(void **)((int)this + 0xbc),param_1);
      if (pvVar4 == (void *)0x0) {
        return 0;
      }
      iVar2 = FUN_004fd2e0();
      *(int *)((int)pvVar4 + 0x2c) = iVar2;
      bVar1 = FUN_0051e9a0(this,*(int *)((int)pvVar4 + 0x20),iVar2,*(int *)((int)pvVar4 + 0x28));
      uVar3 = CONCAT31(extraout_var,bVar1);
    }
    if (uVar3 != 0) {
      *param_2 = *(undefined4 *)((int)pvVar4 + 0x2c);
    }
  }
  return uVar3;
}

