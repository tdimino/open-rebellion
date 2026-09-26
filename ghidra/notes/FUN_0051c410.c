
uint __thiscall FUN_0051c410(void *this,int *param_1)

{
  int *piVar1;
  uint uVar2;
  int iVar3;
  int *piVar4;
  int iVar5;
  uint uVar6;
  
  piVar4 = param_1;
  if ((param_1 != (int *)0x0) && (*(int *)((int)this + 4) != 0)) {
    while( true ) {
      piVar1 = *(int **)((int)this + 0xc);
      iVar5 = *(int *)(*piVar1 + 4);
      if (*(int *)((int)piVar1 + iVar5 + 0x34) < 0) {
        FUN_00615ee0((LPCRITICAL_SECTION)((int)piVar1 + iVar5 + 0x38));
      }
      *(undefined4 *)((int)piVar1 + iVar5 + 8) = 0;
      if (*(int *)((int)piVar1 + iVar5 + 0x34) < 0) {
        FUN_00615ef0((LPCRITICAL_SECTION)((int)piVar1 + iVar5 + 0x38));
      }
      FUN_00616900(*(void **)((int)this + 0xc),0);
      param_1 = (int *)(**(code **)(*piVar4 + 0xc))();
      FUN_005f4d00(*(void **)((int)this + 0xc),&param_1);
      (**(code **)(*piVar4 + 8))(*(undefined4 *)((int)this + 0xc));
      iVar5 = *(int *)(*(int *)(**(int **)((int)this + 0xc) + 4) + 4 +
                      (int)*(int **)((int)this + 0xc));
      uVar6 = *(uint *)(iVar5 + 0x1c);
      uVar2 = *(uint *)(iVar5 + 0x18);
      if (uVar6 < uVar2) {
        iVar5 = 0;
      }
      else {
        iVar5 = uVar6 - uVar2;
      }
      *(int *)((int)this + 0x14) = iVar5;
      uVar6 = (uint)(iVar5 != 0);
      if (uVar6 == 0) break;
      iVar3 = *(int *)((int)this + 0x10);
      if (iVar5 == iVar3) {
        uVar6 = FUN_0051c5c0(this,iVar3 * 2,0);
      }
      if (uVar6 == 0) {
        return 0;
      }
      if (iVar5 != iVar3) {
        return uVar6;
      }
    }
  }
  return 0;
}

