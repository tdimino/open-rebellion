
void __thiscall FUN_00420580(void *this,uint *param_1)

{
  void *this_00;
  bool bVar1;
  bool bVar2;
  bool bVar3;
  int iVar4;
  LPCSTR pCVar5;
  uint uVar6;
  uint uVar7;
  int iVar8;
  int iVar9;
  tagSIZE *ptVar10;
  uint local_44;
  tagSIZE local_34;
  uint local_28;
  undefined4 local_24 [3];
  undefined4 auStack_18 [3];
  undefined4 local_c [3];
  
  iVar8 = 0;
  local_28 = 0;
  bVar1 = false;
  uVar6 = *(uint *)((int)this + 300);
  iVar4 = *(int *)((int)this + 0xdc) * 4;
  this_00 = (void *)((int)this + 0x10c);
  bVar2 = false;
  FUN_005f3e30(this_00,(int)this + 0xa0);
  if (0 < *(int *)((int)this + 0x174)) {
    do {
      FUN_005f3e70(this_00,local_24);
      FUN_005f3e00(local_24);
      iVar8 = iVar8 + 1;
    } while (iVar8 < *(int *)((int)this + 0x174));
  }
  local_44 = *(uint *)((int)this + 0x174);
  FUN_005f3e10((void *)((int)this + 0x100),(int)this_00);
  iVar8 = FUN_005f3fa0((int)this_00);
  do {
    if ((iVar8 == 0) || (bVar1)) goto LAB_004208e6;
    bVar3 = false;
    if (((int)local_28 <= (int)param_1[2]) &&
       ((int)param_1[2] < (int)(*(int *)((int)this + 200) + local_28))) {
      bVar2 = true;
      param_1[2] = local_28;
    }
    do {
      iVar8 = FUN_005f3fa0((int)this + 0x10c);
      uVar7 = uVar6;
      if ((iVar8 == 0) || (bVar1)) break;
      iVar8 = FUN_00420930((int)this + 0x100,(void *)((int)this + 0x10c));
      if (iVar8 == 0) {
        local_44 = local_44 + 1;
        iVar8 = FUN_005f3f80((int)this + 0x10c);
        if (iVar8 == 9) {
          uVar7 = uVar6 + 1 + (iVar4 - (int)(uVar6 + 1) % iVar4);
          if (*(int *)((int)this + 0x130) < (int)uVar7) goto LAB_00420840;
          if ((bVar2) && ((int)param_1[1] <= (int)uVar7)) {
            bVar1 = true;
            param_1[1] = uVar6;
            *param_1 = local_44;
          }
        }
        else if (iVar8 == 10) {
LAB_00420840:
          bVar3 = true;
        }
        else if (iVar8 == 0x20) {
          uVar7 = uVar6 + *(int *)((int)this + 0x128);
          if (*(int *)((int)this + 0x130) < (int)uVar7) {
            bVar3 = true;
            uVar7 = uVar6;
          }
          else if ((bVar2) && ((int)param_1[1] <= (int)uVar7)) {
            bVar1 = true;
            param_1[1] = uVar7 - *(int *)((int)this + 0x128);
            *param_1 = local_44;
          }
        }
        FUN_005f3e70((void *)((int)this + 0x100),local_c);
        FUN_005f3e00(local_c);
      }
      else {
        ptVar10 = &local_34;
        iVar9 = iVar8;
        pCVar5 = (LPCSTR)FUN_00583c40((int)this + 0x100);
        GetTextExtentPoint32A(*(HDC *)((int)this + 0x11c),pCVar5,iVar9,ptVar10);
        if ((*(int *)((int)this + 0x130) < (int)(uVar6 + local_34.cx)) &&
           (*(int *)((int)this + 300) < (int)uVar6)) {
          uVar7 = uVar6 - *(int *)((int)this + 0x128);
          break;
        }
        if (*(int *)((int)this + 0x130) - *(int *)((int)this + 300) < local_34.cx) {
          do {
            if (iVar8 < 2) break;
            iVar8 = iVar8 + -1;
            FUN_005f3ef0((void *)((int)this + 0x10c),local_24);
            FUN_005f3e00(local_24);
            ptVar10 = &local_34;
            iVar9 = iVar8;
            pCVar5 = (LPCSTR)FUN_00583c40((int)this + 0x100);
            GetTextExtentPoint32A(*(HDC *)((int)this + 0x11c),pCVar5,iVar9,ptVar10);
          } while (*(int *)((int)this + 0x130) - *(int *)((int)this + 300) < local_34.cx);
        }
        if ((bVar2) && ((int)param_1[1] <= (int)(uVar6 + local_34.cx))) {
          do {
            if (iVar8 < 1) break;
            iVar8 = iVar8 + -1;
            FUN_005f3ef0((void *)((int)this + 0x10c),auStack_18);
            FUN_005f3e00(auStack_18);
            ptVar10 = &local_34;
            iVar9 = iVar8;
            pCVar5 = (LPCSTR)FUN_00583c40((int)this + 0x100);
            GetTextExtentPoint32A(*(HDC *)((int)this + 0x11c),pCVar5,iVar9,ptVar10);
          } while ((int)param_1[1] <= (int)(uVar6 + local_34.cx));
          bVar1 = true;
          param_1[1] = uVar6 + local_34.cx;
          *param_1 = local_44 + iVar8;
        }
        uVar7 = uVar6 + local_34.cx;
        local_44 = local_44 + iVar8;
        iVar8 = FUN_005f3e50((int)this + 0x10c);
        FUN_005f3e10((void *)((int)this + 0x100),iVar8);
      }
      uVar6 = uVar7;
    } while (!bVar3);
    if ((bVar2) && (!bVar1)) {
      bVar1 = true;
      param_1[1] = uVar7;
      *param_1 = local_44 - 1;
LAB_004208e6:
      iVar4 = FUN_005f3fa0((int)this + 0x10c);
      if ((iVar4 == 0) && (!bVar1)) {
        uVar6 = FUN_005f3070((int)this + 0xa0);
        *param_1 = uVar6 & 0xffff;
        param_1[2] = local_28;
        param_1[1] = *(uint *)((int)this + 300);
      }
      return;
    }
    uVar6 = *(uint *)((int)this + 300);
    local_28 = local_28 + *(int *)((int)this + 200);
    iVar8 = FUN_005f3fa0((int)this + 0x10c);
  } while( true );
}

