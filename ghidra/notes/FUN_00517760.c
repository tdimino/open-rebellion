
uint __cdecl FUN_00517760(int *param_1,void *param_2)

{
  int *this;
  void *pvVar1;
  int iVar2;
  undefined *puVar3;
  int *piVar4;
  uint uVar5;
  int iStack_28;
  undefined4 auStack_24 [3];
  undefined4 auStack_18 [3];
  void *pvStack_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  this = param_1;
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_00642948;
  pvStack_c = ExceptionList;
  uVar5 = 1;
  ExceptionList = &pvStack_c;
  iVar2 = (**(code **)(*param_1 + 0x48))();
  if ((iVar2 != 0) && ((*(byte *)(this + 0x14) & 0x40) != 0)) {
    iVar2 = this[0xb];
    uVar5 = (uint)(iVar2 != 0);
    if (iVar2 != 0) {
      puVar3 = FUN_004f62d0((int)this);
      param_1 = (int *)FUN_005f3390((void *)(iVar2 + 0x34),(int)puVar3);
      if (param_1 == (int *)0x0) {
        FUN_00518750((int)this,(int *)&param_1);
        pvVar1 = param_2;
        if (param_1 != (int *)0x0) {
          piVar4 = FUN_00402d80(this,&param_2);
          uStack_4 = 0;
          FUN_005302c0(param_1,piVar4,&iStack_28);
          uStack_4 = 0xffffffff;
          FUN_00619730();
        }
        FUN_005f2f50(auStack_24);
        uStack_4 = 1;
        FUN_005f35b0(auStack_18,(char *)&lpString_006a7f40);
        uStack_4._0_1_ = 2;
        FUN_005f30d0(auStack_24,iVar2 + 0x34);
        FUN_005f30d0(auStack_24,(int)auStack_18);
        FUN_005f31a0(auStack_24,iStack_28);
        uStack_4 = CONCAT31(uStack_4._1_3_,1);
        FUN_005f2ff0(auStack_18);
        uVar5 = FUN_004f6e60(this,(int)auStack_24,pvVar1);
        uStack_4 = 0xffffffff;
        FUN_005f2ff0(auStack_24);
      }
    }
  }
  ExceptionList = pvStack_c;
  return uVar5;
}

