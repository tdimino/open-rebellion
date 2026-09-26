
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined4 __thiscall FUN_005b8570(void *this,int param_1)

{
  uint uVar1;
  int iVar2;
  uint uVar3;
  float10 fVar4;
  int iVar5;
  
  if (*(uint *)((int)this + 0xc) < *(uint *)((int)this + 0x10)) {
    *(int *)((int)this + 8) = *(int *)((int)this + 8) + 1;
    *(uint *)((int)this + 0xc) = *(uint *)((int)this + 0xc) + 1;
    FUN_005c5f20(*(void **)((int)this + 0x1c),param_1);
    *(undefined4 *)(param_1 + 0x164) = *(undefined4 *)((int)this + 4);
    FUN_005b69d0(param_1);
    if (*(float *)((int)this + 0x14) == _DAT_0066c690) {
      iVar2 = FUN_00597530();
      if (*(int *)(iVar2 + 0x668) == 0) {
        iVar2 = FUN_005aaf90();
        uVar1 = *(uint *)(iVar2 + 0x87c);
        iVar5 = 8;
        iVar2 = 0;
        FUN_005aaf90();
        uVar3 = FUN_005a8a70(iVar2,iVar5);
        fVar4 = FUN_005b8b00((int)this);
        *(float *)((int)this + 0x14) = (float)(fVar4 + (float10)uVar3 + (float10)uVar1);
      }
    }
    return 1;
  }
  return 0;
}

