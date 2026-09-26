
void __thiscall FUN_00457690(void *this,HDC param_1)

{
  RECT *lprcSrc2;
  int iVar1;
  int *piVar2;
  int iVar3;
  void *this_00;
  HDC hdc;
  BOOL BVar4;
  HGDIOBJ pvVar5;
  char *pcVar6;
  uint uVar7;
  void *this_01;
  undefined4 *puVar8;
  uint *puVar9;
  int iStack_5c;
  undefined4 auStack_4c [3];
  undefined4 auStack_40 [3];
  undefined4 auStack_34 [3];
  undefined4 auStack_28 [3];
  tagRECT tStack_1c;
  void *local_c;
  undefined1 *puStack_8;
  int iStack_4;
  
  iStack_4 = 0xffffffff;
  puStack_8 = &LAB_00630460;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  piVar2 = FUN_004a25c0(this);
  if (piVar2 != (int *)0x0) {
    iVar3 = *(int *)((int)this + 300);
    iVar1 = *(int *)((int)this + 0x128);
    if (*(int *)(*(int *)((int)this + 0x14c) + 0xb8) == *(int *)((int)this + 0x18)) {
      FUN_005fc140(*(void **)((int)this + 0x180),param_1,'@',0xcc0020,iVar1,iVar3,0,0,
                   *(int *)((int)this + 0x130) - iVar1,*(int *)((int)this + 0x134) - iVar3);
    }
    else {
      FUN_005fc140(*(void **)((int)this + 0x184),param_1,'@',0xcc0020,iVar1,iVar3,0,0,
                   *(int *)((int)this + 0x130) - iVar1,*(int *)((int)this + 0x134) - iVar3);
    }
    iVar3 = *(int *)(*(int *)((int)this + 0x16c) + 0x94);
    if (iVar3 == 0) {
      uVar7 = 0;
    }
    else {
      uVar7 = *(uint *)(iVar3 + 0x24);
    }
    iVar3 = FUN_00604500((void *)(*(int *)((int)this + 0x16c) + 0x6c),uVar7);
    uVar7 = *(uint *)((int)this + 0x160);
    if (uVar7 == 0x67) {
      this_00 = (void *)FUN_006073d0(this);
      hdc = CreateCompatibleDC(param_1);
      puVar8 = (undefined4 *)((int)this + 0x1c0);
      iStack_5c = 3;
      do {
        lprcSrc2 = (RECT *)(puVar8 + -8);
        BVar4 = IntersectRect(&tStack_1c,(RECT *)&stack0x00000008,lprcSrc2);
        if (BVar4 != 0) {
          FUN_005fcc30(this_00,*(uint **)((int)this + 0x194),0,0,lprcSrc2->left,puVar8[-7],
                       (undefined4 *)0x0,0);
          FUN_005fd0f0(*(void **)((int)this + 0x188),*(uint **)((int)this + 0x194),0,0);
          if ((*(byte *)(puVar8 + 4) & 1) == 0) {
            puVar9 = *(uint **)((int)this + 0x194);
            this_01 = *(void **)((int)this + 400);
          }
          else {
            puVar9 = *(uint **)((int)this + 0x194);
            this_01 = *(void **)((int)this + 0x18c);
          }
          FUN_005fd0f0(this_01,puVar9,0,0);
          pvVar5 = (HGDIOBJ)FUN_005fc130(*(int *)((int)this + 0x194));
          pvVar5 = SelectObject(hdc,pvVar5);
          FUN_00601ce0((void *)*puVar8,hdc);
          if ((puVar8[-4] & 0xff000000) != 0) {
            FUN_005fd0f0((void *)puVar8[-1],*(uint **)((int)this + 0x194),0x28,0xf);
            FUN_00601ce0((void *)puVar8[2],hdc);
          }
          FUN_00601ce0((void *)puVar8[1],hdc);
          FUN_00601ce0((void *)puVar8[3],hdc);
          SelectObject(hdc,pvVar5);
          FUN_005fc140(*(void **)((int)this + 0x194),param_1,'@',0xcc0020,lprcSrc2->left,puVar8[-7],
                       0,0,0,0);
        }
        puVar8 = puVar8 + 0xe;
        iStack_5c = iStack_5c + -1;
      } while (iStack_5c != 0);
      DeleteDC(hdc);
      FUN_005fc140(*(void **)((int)this + 0x198),param_1,'@',0xcc0020,6,0x47,0,0,0,0);
      FUN_005f2f50(auStack_4c);
      iStack_4 = 0;
      iVar3 = FUN_0052c8c0(piVar2,*(undefined4 *)((int)this + 0x148),1);
      FUN_005f31a0(auStack_4c,iVar3);
      FUN_005f35b0(auStack_40,&DAT_006a872c);
      iStack_4._0_1_ = 1;
      FUN_005f30d0(auStack_4c,(int)auStack_40);
      iStack_4._0_1_ = 0;
      FUN_005f2ff0(auStack_40);
      iVar3 = FUN_0052c8c0(piVar2,*(undefined4 *)((int)this + 0x148),3);
      FUN_005f31a0(auStack_4c,iVar3);
      FUN_00601b30(*(void **)((int)this + 0x19c),6,0x77);
      pcVar6 = (char *)FUN_00583c40((int)auStack_4c);
      FUN_00601aa0(*(void **)((int)this + 0x19c),pcVar6);
      FUN_00601ce0(*(void **)((int)this + 0x19c),param_1);
      uVar7 = FUN_005f3070((int)auStack_4c);
      FUN_005f3c30(auStack_4c,0,uVar7 & 0xffff);
      iVar3 = FUN_0052c5a0(piVar2,*(undefined4 *)((int)this + 0x148),1);
      FUN_005f31a0(auStack_4c,iVar3);
      FUN_005f35b0(auStack_34,&DAT_006a872c);
      iStack_4._0_1_ = 2;
      FUN_005f30d0(auStack_4c,(int)auStack_34);
      iStack_4._0_1_ = 0;
      FUN_005f2ff0(auStack_34);
      iVar3 = FUN_0052c5a0(piVar2,*(undefined4 *)((int)this + 0x148),3);
      FUN_005f31a0(auStack_4c,iVar3);
      FUN_00601b30(*(void **)((int)this + 0x19c),6,200);
      pcVar6 = (char *)FUN_00583c40((int)auStack_4c);
      FUN_00601aa0(*(void **)((int)this + 0x19c),pcVar6);
      FUN_00601ce0(*(void **)((int)this + 0x19c),param_1);
      uVar7 = FUN_005f3070((int)auStack_4c);
      FUN_005f3c30(auStack_4c,0,uVar7 & 0xffff);
      iVar3 = FUN_0052c270(piVar2,*(undefined4 *)((int)this + 0x148),1);
      FUN_005f31a0(auStack_4c,iVar3);
      FUN_005f35b0(auStack_28,&DAT_006a872c);
      iStack_4._0_1_ = 3;
      FUN_005f30d0(auStack_4c,(int)auStack_28);
      iStack_4 = (uint)iStack_4._1_3_ << 8;
      FUN_005f2ff0(auStack_28);
      iVar3 = FUN_0052c270(piVar2,*(undefined4 *)((int)this + 0x148),3);
      FUN_005f31a0(auStack_4c,iVar3);
      FUN_00601b30(*(void **)((int)this + 0x19c),6,0x118);
      pcVar6 = (char *)FUN_00583c40((int)auStack_4c);
      FUN_00601aa0(*(void **)((int)this + 0x19c),pcVar6);
      FUN_00601ce0(*(void **)((int)this + 0x19c),param_1);
      iStack_4 = 0xffffffff;
      FUN_005f2ff0(auStack_4c);
    }
    else if ((0x67 < uVar7) && (uVar7 < 0x6d)) {
      FUN_00601aa0(*(void **)((int)this + 0x17c),*(char **)(iVar3 + 0x84));
      FUN_00601ce0(*(void **)((int)this + 0x17c),param_1);
    }
  }
  ExceptionList = local_c;
  return;
}

