from flask import (Blueprint, render_template)

router = Blueprint('bookkeeper', __name__, url_prefix='/bookkeeper')


@router.route('/statements/<token>', methods=['GET'])
def show_statement(token):
    return render_template('bookkeeper/statements/show.html', token=token)
