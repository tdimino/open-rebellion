
void __thiscall FUN_0044aa20(void *this,int param_1)

{
  int iVar1;
  bool bVar2;
  int iVar3;
  int *this_00;
  uint uVar4;
  void *pvVar5;
  void *pvVar6;
  uint *puVar7;
  char *pcVar8;
  uint uStack_84;
  void *pvStack_80;
  undefined4 auStack_7c [3];
  int iStack_70;
  int aiStack_6c [2];
  undefined1 auStack_64 [8];
  undefined4 auStack_5c [6];
  undefined1 auStack_44 [4];
  int aiStack_40 [2];
  _RTL_CRITICAL_SECTION _Stack_38;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 uStack_4;
  
  uStack_4 = 0xffffffff;
  puStack_8 = &LAB_0062f3e3;
  local_c = ExceptionList;
  bVar2 = false;
  ExceptionList = &local_c;
  for (iVar3 = thunk_FUN_005f5060(param_1); iVar3 != 0; iVar3 = *(int *)(iVar3 + 0x10)) {
    this_00 = FUN_004f2d10(*(int *)(*(int *)((int)this + 0x114) + 0x9c),(uint *)(iVar3 + 0x24));
    if (this_00 != (int *)0x0) {
      uVar4 = (**(code **)(*this_00 + 0x30))();
      pvStack_80 = (void *)FUN_00618b70(0x28);
      uStack_4 = 0;
      if (pvStack_80 == (void *)0x0) {
        pvVar5 = (void *)0x0;
      }
      else {
        pvVar5 = FUN_005fbd20(pvStack_80,*(undefined4 *)((int)this + 0x1d4),(uVar4 & 0xfff) + 0x4000
                              ,10);
      }
      uStack_4 = 0xffffffff;
      pvVar6 = (void *)FUN_00618b70(0x68);
      uStack_4 = 1;
      pvStack_80 = pvVar6;
      if (pvVar6 == (void *)0x0) {
        pvVar6 = (void *)0x0;
      }
      else {
        puVar7 = FUN_004025b0(this_00,&uStack_84);
        bVar2 = true;
        uStack_4 = CONCAT31(uStack_4._1_3_,2);
        pvVar6 = FUN_0060ba00(pvVar6,*puVar7 >> 0x18);
      }
      uStack_4 = 1;
      if (bVar2) {
        bVar2 = false;
        FUN_00619730();
      }
      uStack_4 = 0xffffffff;
      *(undefined4 *)((int)pvVar6 + 0x3c) = 4;
      FUN_0060be60(pvVar6,(int)pvVar5,0);
      FUN_005f2f50(auStack_7c);
      uStack_4 = 3;
      FUN_00615db0(&iStack_70,1);
      uStack_4 = CONCAT31(uStack_4._1_3_,4);
      FUN_00616900(auStack_64,0);
      iVar1 = *(int *)(iStack_70 + 4);
      if (*(int *)((int)aiStack_40 + iVar1 + 4) < 0) {
        FUN_00615ee0((LPCRITICAL_SECTION)((int)&_Stack_38.DebugInfo + iVar1));
      }
      *(undefined4 *)((int)aiStack_6c + iVar1 + 4) = 0;
      if (*(int *)((int)aiStack_40 + iVar1 + 4) < 0) {
        FUN_00615ef0((LPCRITICAL_SECTION)((int)&_Stack_38.DebugInfo + iVar1));
      }
      auStack_44[*(int *)(iStack_70 + 4)] = 0x30;
      *(undefined4 *)((int)aiStack_40 + *(int *)(iStack_70 + 4)) = 3;
      FUN_00617540(auStack_64,*(int *)(iVar3 + 0x20));
      FUN_00616110(auStack_64,&DAT_006a874c);
      *(undefined4 *)((int)aiStack_40 + *(int *)(iStack_70 + 4)) = 4;
      FUN_00617540(auStack_64,*(int *)(iVar3 + 0x1c));
      FUN_00615f00(auStack_64,0);
      pcVar8 = (char *)FUN_00615680(*(int *)((int)aiStack_6c + *(int *)(iStack_70 + 4)));
      FUN_005f35e0(auStack_7c,pcVar8);
      FUN_00615660(*(void **)((int)aiStack_6c + *(int *)(iStack_70 + 4)),0);
      pcVar8 = (char *)FUN_00583c40((int)auStack_7c);
      FUN_005f35e0((void *)((int)pvVar6 + 0x14),pcVar8);
      FUN_005f59f0(*(void **)((int)this + 0x11c),(int)pvVar6);
      uStack_4 = CONCAT31(uStack_4._1_3_,3);
      FUN_00615ea0((int)auStack_5c);
      FUN_00615440(auStack_5c);
      uStack_4 = 0xffffffff;
      FUN_005f2ff0(auStack_7c);
    }
  }
  ExceptionList = local_c;
  return;
}

