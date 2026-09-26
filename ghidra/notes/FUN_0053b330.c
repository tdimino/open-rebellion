
undefined4 __thiscall FUN_0053b330(void *this,uint param_1,void *param_2)

{
  int iVar1;
  bool bVar2;
  uint uVar3;
  int iVar4;
  bool bVar5;
  
  bVar5 = true;
  if (param_1 != 0) {
    uVar3 = FUN_0053fab0(0x394,*(uint *)(*(int *)((int)this + 0x54) + 0x20),this,
                         (int *)(*(int *)((int)this + 0x54) + 0x18),param_2);
    bVar5 = uVar3 != 0;
  }
  uVar3 = FUN_0053fa60(0x394,param_1,this,(int *)(*(int *)((int)this + 0x54) + 0x18),param_2);
  if ((uVar3 == 0) || (bVar2 = true, !bVar5)) {
    bVar2 = false;
  }
  iVar4 = 0;
  if (param_1 != 0) {
    iVar1 = *(int *)((int)this + 0x54);
    iVar4 = FUN_004fd340();
    iVar4 = iVar4 + *(int *)(iVar1 + 0x20);
  }
  iVar4 = FUN_0053a770(this,iVar4,param_2);
  if ((iVar4 != 0) && (bVar2)) {
    return 1;
  }
  return 0;
}

