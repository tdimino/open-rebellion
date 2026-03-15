
int __thiscall FUN_004f7260(void *this,uint param_1,void *param_2)

{
  uint uVar1;
  bool bVar2;
  undefined3 extraout_var;
  int iVar3;
  void *pvVar4;
  uint uVar5;
  void *pvVar6;
  
  bVar2 = FUN_0053a000((int)this);
  iVar3 = CONCAT31(extraout_var,bVar2);
  if (iVar3 != 0) {
    iVar3 = FUN_0053fc90(param_1,0,param_1);
  }
  if (iVar3 != 0) {
    uVar1 = *(uint *)((int)this + 0x40);
    uVar5 = uVar1 >> 8 & 0xff;
    if (uVar5 != param_1) {
      *(uint *)((int)this + 0x40) =
           CONCAT22((short)(uVar1 >> 0x10),(ushort)(byte)uVar1) | (param_1 & 0xff) << 8;
      pvVar6 = param_2;
      pvVar4 = (void *)FUN_00539fd0(this,1);
      FUN_004fa210(pvVar4,pvVar6);
      pvVar6 = param_2;
      pvVar4 = (void *)FUN_00539fd0(this,2);
      FUN_004fa210(pvVar4,pvVar6);
                    /* WARNING: Load size is inaccurate */
      (**(code **)(*this + 0x104))(uVar5,*(undefined1 *)((int)this + 0x41),param_2);
    }
  }
  return iVar3;
}

