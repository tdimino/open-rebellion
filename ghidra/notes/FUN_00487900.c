
void __thiscall FUN_00487900(void *this,void *param_1)

{
  bool bVar1;
  uint uVar2;
  undefined3 extraout_var;
  int iVar3;
  uint uVar4;
  
  if (*(int *)((int)this + 0xc0) != 0) {
    FUN_005f4960(&DAT_006b28b0);
    uVar2 = FUN_00487c20((int)this);
    bVar1 = FUN_00487c30((int)this);
    uVar4 = CONCAT31(extraout_var,bVar1) & uVar2;
    FUN_00520690(param_1,0x110,uVar4);
    FUN_00520690(param_1,0x111,uVar4);
    FUN_00520690(param_1,0x112,uVar4);
    FUN_00520690(param_1,0x115,uVar2);
    FUN_00520690(param_1,0x116,uVar2);
    FUN_00520690(param_1,0x113,1);
    FUN_00520690(param_1,0x11b,uVar2);
    FUN_00520690(param_1,0x114,1);
    FUN_00520690(param_1,0x11e,uVar2);
    if ((DAT_006b28b0 & 0x1000) != 0) {
      iVar3 = FUN_00520720(param_1,0x11b);
      *(undefined4 *)(iVar3 + 0x20) = 1;
    }
    uVar2 = FUN_005f4960(&DAT_006b28b0);
    if ((uVar2 & 0x8000) == 0) {
      iVar3 = FUN_00520720(param_1,0x11e);
      *(undefined4 *)(iVar3 + 0x20) = 1;
    }
    iVar3 = FUN_00439e30(*(void **)((int)this + 0xc0),0x115);
    if (iVar3 != 0) {
      iVar3 = FUN_00520720(param_1,0x115);
      *(undefined4 *)(iVar3 + 0x20) = 1;
    }
    iVar3 = FUN_00439e30(*(void **)((int)this + 0xc0),0x116);
    if (iVar3 != 0) {
      iVar3 = FUN_00520720(param_1,0x116);
      *(undefined4 *)(iVar3 + 0x20) = 1;
    }
    iVar3 = FUN_00439e30(*(void **)((int)this + 0xc0),0x117);
    if (iVar3 != 0) {
      iVar3 = FUN_00520720(param_1,0x117);
      *(undefined4 *)(iVar3 + 0x20) = 1;
    }
  }
  return;
}

